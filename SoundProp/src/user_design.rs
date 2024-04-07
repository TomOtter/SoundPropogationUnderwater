use std::{process::Output, time::Duration};

use crate::ray_trace::{self, Rays};
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

pub enum Source_Type {
    point,
    line,
}

pub struct Simulation {
    pub dt : f64,
    pub sources : Vec<Source>,
    //pub water_type : somefunction,
    //pub boundary_type: somefunction,
}

impl Simulation {

    pub fn add_source(&mut self, min_angle: f64, max_angle: f64, number_of_rays: i32, intensity: f64, frequency: f64, location: [f64;2], source_type: Source_Type) -> () {
        if min_angle >= max_angle {
            eprintln!("Error: Minimum angle must be less than the maximum angle.");
            std::process::exit(1);
        } // Checks if the minimum angle is less than the maximum angle.
        if min_angle.abs() > PI || max_angle.abs() > PI {
            eprintln!("Error: Minimum and maximum angles must be within the range of -π to π.");
            std::process::exit(1);
        } // Checks if the minimum and maximum angles are within the range of +/- PI.

        let new_source: Source = Source {
            source_type : source_type,
            angle_range : [min_angle, max_angle],
            number_of_rays : (number_of_rays as usize),
            intensity : intensity,
            frequency : frequency,
            location : location,
        };
        
        self.sources.push(new_source);
        // Adds new source to an array of sources under the Simulation struct.
    }

    pub fn calculate(&mut self, duration: f64) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
        if self.sources.len() == 0 {
            eprintln!("Error: No sources have been defined. Call 'self.addSource' prior to this function to define a soundwave source.");
            std::process::exit(1);
        } // Ensures that a source has been defined prior to this function.
        
        let SIZE: i32 = (duration / self.dt) as i32;
        let number_of_rays: usize = self.sources.iter().map(|source| source.number_of_rays as usize).sum();
        //Sums 'number_of_rays' across all sources.

        let mut x_output: Vec<Vec<f64>> = Vec::with_capacity(SIZE as usize);
        let mut y_output: Vec<Vec<f64>> = Vec::with_capacity(SIZE as usize);
        // Allocates the x and y ouputs. Inner vector indexes ray count, outer vector indexes time.

        let mut Rays: Rays = ray_trace::Rays {
            angle: Vec::with_capacity(number_of_rays as usize),
            x_pos: Vec::with_capacity(number_of_rays as usize),
            y_pos: Vec::with_capacity(number_of_rays as usize),
            intensity: Vec::with_capacity(number_of_rays as usize),
            frequency: Vec::with_capacity(number_of_rays as usize),
            step_vector: Vec::with_capacity(number_of_rays as usize),//dy,
        }; // Defines the Rays struct with each variable inside having an appendable vector with minimum array size (beneficial for memory).
        
        for i in 0..self.sources.len() {
            self.sources[i].initialise(&mut Rays, self.dt);
        } // Compiles all of the initial data for each ray, from its sources, into one 'Rays' struct.

        Rays.initialise(self.dt);

        for i in 0..SIZE-1 {
            if i != 0{
              Rays.step();  
            }
            x_output.push( Rays.x_pos.clone() );
            y_output.push( Rays.y_pos.clone() );
        }
        (x_output, y_output)
    }
}




pub struct Source {
    source_type : Source_Type,
    angle_range : [f64;2],
    number_of_rays : usize,
    intensity : f64,
    frequency : f64,
    location : [f64;2],
}

impl Source {
    pub fn initialise(&mut self, initial_rays: &mut Rays, dt: f64) {
        match self.source_type {
            Source_Type::point => {
                let angle_spacing: f64 = (self.angle_range[1] - self.angle_range[0]) / (self.number_of_rays as f64);

                for i in 0..self.number_of_rays-1 {
                    initial_rays.angle.push( self.angle_range[0] + (angle_spacing * i as f64) );
                } // Evenly spaces out the arrays at the source between the given bounds and appends to the initial ray angles struct.

                initial_rays.x_pos.extend( vec![self.location[0];self.number_of_rays-1] );
                initial_rays.y_pos.extend( vec![self.location[1];self.number_of_rays-1] );
                initial_rays.intensity.extend( vec![self.intensity;self.number_of_rays-1] );
                initial_rays.step_vector.extend( vec![dt;self.number_of_rays-1] );
                initial_rays.frequency.extend( vec![self.frequency;self.number_of_rays-1] );
            }
            Source_Type::line => {
                println!("Not yet implemented");
            }
        } 
    }
}
