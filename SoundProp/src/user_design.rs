use core::num;
use std::{process::Output, time::Duration};
use std::fs::File;
use crate::ray_trace::{self, Rays};
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;
use std::io::prelude::*;

pub enum SourceType {
    Point,
    Line,
}

//                                               MARK: Simulation Struct
pub struct Simulation {
    dt : f64,
    sources : Vec<Source>,
    //pub water_type : somefunction,
    //pub boundary_type: somefunction,
}

impl Simulation {

    pub fn initialise(dt: f64) -> Self {
        if dt <= 0.0 {
            eprintln!("Error: dt must be a positive, non-zero, float.");
            std::process::exit(1);
        } // Ensures that dt is a positive, non-zero, float.
        Self {
            dt : dt,
            sources : Vec::new(),
        }
    }

    pub fn add_source(&mut self, min_angle: f64, max_angle: f64, number_of_rays: i32, intensity: f64, frequency: f64, location: [f64;2], source_type: SourceType) -> () {
        if min_angle >= max_angle {
            eprintln!("Error: Minimum angle must be less than the maximum angle.");
            std::process::exit(1);
        } // Checks if the minimum angle is less than the maximum angle.
        if min_angle.abs() > PI || max_angle.abs() > PI {
            eprintln!("Error: Minimum and maximum angles must be within the range of -π to π.");
            std::process::exit(1);
        } // Checks if the minimum and maximum angles are within the range of +/- PI.

        let new_source: Source = Source::initialise(source_type, min_angle, max_angle, 
            number_of_rays as usize, intensity, frequency, location);
        
        self.sources.push(new_source);
        // Adds new source to an array of sources under the Simulation struct.
    }

    pub fn calculate(&mut self, duration: f64) -> () {
        if self.sources.len() == 0 {
            eprintln!("Error: No sources have been defined. Call 'self.addSource' prior to this function to define a soundwave source.");
            std::process::exit(1);
        } // Ensures that a source has been defined prior to this function.
        
        let size: i32 = (duration / self.dt) as i32;
        let number_of_rays: usize = self.sources.iter().map(|source| source.number_of_rays as usize).sum();
        //Sums 'number_of_rays' across all sources.

        let mut x_output: Vec<Vec<f64>> = Vec::with_capacity(size as usize);
        let mut y_output: Vec<Vec<f64>> = Vec::with_capacity(size as usize);
        // Allocates the x and y ouputs. Inner vector indexes ray count, outer vector indexes time.

        let mut rays: Rays = ray_trace::Rays::initialise(number_of_rays);
        // Defines the Rays struct with each variable inside having an appendable vector with minimum array size (beneficial for memory).
        
        for i in 0..self.sources.len() {
            self.sources[i].create_rays(&mut rays, self.dt);
        } // Compiles all of the initial data for each ray, from its sources, into one 'Rays' struct.

        rays.bound_angles(self.dt);

        for i in 0..size-1 {
            if i != 0{
              rays.step();  
            } // Done to ensure that the initial positions of the rays is not overwritten in the output file.
            let (x_pos, y_pos) = rays.output_position();
            x_output.push( x_pos );
            y_output.push( y_pos );
        }
        self.output(x_output, y_output);
    }

    fn output(&mut self, xdata: Vec<Vec<f64>>, ydata: Vec<Vec<f64>>) {
        if xdata.len() != ydata.len() {
            eprintln!("Error: Length of 'xdata' array does not match with length of 'ydata' array.");
            std::process::exit(1);
        } 
    
        for i in 0..xdata.len() - 1 {
            if xdata[i].len() != ydata[i].len() {
                eprintln!(
                    "Error: Length of 'xdata{0}' array does not match with length of 'ydata{0}' array.", i);
                std::process::exit(1);
            } // Ensures that each ray has both a x and y position at each time step.
    
            let mut output = String::new(); // Create a new string to hold the output for this iteration
    
            for j in 0..xdata[i].len() - 1 {
                let xpos: f64 = xdata[i][j];
                let ypos: f64 = ydata[i][j];
                output.push_str(&format!("{} {}\n", xpos, -1.0 * ypos)); // Append position data to the output string
            }
    
            let file_name = format!("dataset{}.txt", i);
    
            let mut file = match File::create(&file_name) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error creating file {}: {}", &file_name, err);
                    std::process::exit(1);
                }
            };
            if let Err(err) = file.write_all(output.as_bytes()) {
                eprintln!("Error writing to file {}: {}", &file_name, err);
                std::process::exit(1);
            } // Creats a file for each time step and writes the x and y positions of each ray. Terminates program on failure.
        }
    }

}


//                                                 MARK: Source Struct
pub struct Source {
    source_type : SourceType,
    angle_range : [f64;2],
    number_of_rays : usize,
    intensity : f64,
    frequency : f64,
    location : [f64;2],
}

impl Source {
    pub fn initialise(source_type: SourceType, min_angle: f64, max_angle: f64,
    number_of_rays: usize, intensity: f64, frequency: f64, location: [f64;2]) -> Self {
        Self {
            source_type : source_type,
            angle_range : [min_angle, max_angle],
            number_of_rays : number_of_rays,
            intensity : intensity,
            frequency : frequency,
            location : location,
        }
    }

    pub fn create_rays(&mut self, initial_rays: &mut Rays, dt: f64) {
        let mut initial_angles: Vec<f64> = Vec::with_capacity(self.number_of_rays);
        match self.source_type {
            SourceType::Point => {
                let angle_spacing: f64 = (self.angle_range[1] - self.angle_range[0]) / (self.number_of_rays as f64);

                for i in 0..self.number_of_rays-1 {
                    initial_angles.push( self.angle_range[0] + (angle_spacing * i as f64) );
                } // Evenly spaces out the arrays at the source between the given bounds and appends to the initial ray angles struct.

                /* initial_rays.x_pos.extend( vec![self.location[0];self.number_of_rays-1] );
                initial_rays.y_pos.extend( vec![-1.0 * self.location[1];self.number_of_rays-1] );
                initial_rays.intensity.extend( vec![self.intensity;self.number_of_rays-1] );
                initial_rays.step_vector.extend( vec![dt;self.number_of_rays-1] );
                initial_rays.frequency.extend( vec![self.frequency;self.number_of_rays-1] ); */
                initial_rays.create_rays(initial_angles,
                    vec![self.location[0] ; self.number_of_rays-1],
                    vec![-1.0 * self.location[1];self.number_of_rays-1],
                    vec![self.intensity;self.number_of_rays-1],
                    vec![self.frequency;self.number_of_rays-1],
                    vec![dt;self.number_of_rays-1])
            }
            SourceType::Line => {
                println!("Not yet implemented");
            }
        } 
    }

}
