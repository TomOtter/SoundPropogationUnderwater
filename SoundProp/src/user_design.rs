use core::num;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    process::{Command, Output},
    time::Duration,
};
use crate::ray_trace::{self, Rays};
pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

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
    } // Initialisation function to define the fields inside of Simulation after undergoing necessary error checks.

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

        for i in 0..size {
            if i != 0{
              rays.step();  
            } // Done to ensure that the initial positions of the rays is not overwritten in the output file.
            let (x_pos, y_pos) = rays.output_position();
            x_output.push( x_pos );
            y_output.push( y_pos );
        } // Time loop which pushes each ray by one step and outputs the new positions each iteration.
        self.output(x_output, y_output);
    }

//                                                    MARK: Outputs

    fn create_folder(&mut self, folder_path: &str) -> () {
        if !Path::new(folder_path).exists() {
            match std::fs::create_dir(folder_path) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Error creating directory {}: {}", folder_path, err);
                    std::process::exit(1);
                }
            }
        } // Check if the folder exists, if not creates it
    }

    fn output(&mut self, xdata: Vec<Vec<f64>>, ydata: Vec<Vec<f64>>) {
        if xdata.len() != ydata.len() {
            eprintln!("Error: Length of 'xdata' array does not match with length of 'ydata' array.");
            std::process::exit(1);
        } // Ensure the length of 'xdata' matches with 'ydata'
    
        for i in 0..xdata.len() {
            if xdata[i].len() != ydata[i].len() {
                eprintln!("Error: Length of 'xdata{0}' array does not match with length of 'ydata{0}' array.", i);
                std::process::exit(1);
            } // Ensure the length of 'xdata[i]' matches with 'ydata[i]'
    
            let mut output = String::new();
            // Create a string to hold the output for this iteration
    
            for j in 0..xdata[i].len() {
                let xpos: f64 = xdata[i][j];
                let ypos: f64 = ydata[i][j];
                output.push_str(&format!("{} {}\n", xpos, -1.0 * ypos));
                // Append position data to the output string

            } // Loop through each position in the current time step
    
            let folder_path = "./outputdata";
            // Define the folder path where output files will be stored

            self.create_folder(folder_path);

            let file_name = format!("{}/dataset{}.txt", folder_path, i);
            // Define the file name with the folder path and the index 'i'
    
            let mut file = match File::create(&file_name) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Error creating file {}: {}", &file_name, err);
                    std::process::exit(1);
                }
            }; // Create or open the file for writing
    
            if let Err(err) = file.write_all(output.as_bytes()) {
                eprintln!("Error writing to file {}: {}", &file_name, err);
                std::process::exit(1);
            } // Write the output string to the file

        } // Loop through each time step
    }

    pub fn gif(&mut self) -> Output {
        let txt_files = match fs::read_dir("outputdata") {
            Ok(entries) => {
                entries.filter_map(|entry| {
                    if let Ok(entry) = entry {
                        if let Some(extension) = entry.path().extension() {
                            if extension == "txt" {
                                return Some(entry.path());
                            }
                        }
                    }
                    None
                })
                .collect::<Vec<_>>()
            }
            Err(e) => {
                eprintln!("Error: Failed to read directory: {}", e);
                std::process::exit(1);
            } // Terminates the program if directory 'outputdata' is not detected
        };
    
        if txt_files.is_empty() {
            eprintln!("Error: No .txt files found in the outputdata folder");
            std::process::exit(1);
        } // Terminates the program if the directory does not contain .txt files.
        
        self.create_folder("./outputImages");

        let length = txt_files.len();
        let cmd = format!("runGifMAker.bat {} ",length );


    
        if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &cmd ])
                .output()
                .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&cmd)
                .output()
                .expect("failed to execute process")
        }



        // Execute the command to generate the GIF, with OS check
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
        if number_of_rays <= 0 {
            eprintln!("Error: number_of_rays must be a positive, non-zero, integer value.");
            std::process::exit(1);
        }
        if intensity <= 0.0 {
            eprintln!("Error: intensity must be a positive, non-zero, float value.");
            std::process::exit(1);
        }
        if frequency <= 0.0 {
            eprintln!("Error: frequency must be a positive, non-zero, float value.");
            std::process::exit(1);
        }
        Self {
            source_type : source_type,
            angle_range : [min_angle, max_angle],
            number_of_rays : number_of_rays,
            intensity : intensity,
            frequency : frequency,
            location : location,
        }
    } // Initialisation function to define the fields inside of Struct after undergoing necessary error checks.

    pub fn create_rays(&mut self, initial_rays: &mut Rays, dt: f64) {
        let mut initial_angles: Vec<f64> = Vec::with_capacity(self.number_of_rays);
        match self.source_type {
            SourceType::Point => {
                let angle_spacing: f64 = (self.angle_range[1] - self.angle_range[0]) / (self.number_of_rays as f64);

                for i in 0..self.number_of_rays {
                    initial_angles.push( self.angle_range[0] + (angle_spacing * i as f64) );
                } // Evenly spaces out the arrays at the source between the given bounds and appends to the initial ray angles struct.

                /* initial_rays.x_pos.extend( vec![self.location[0];self.number_of_rays-1] );
                initial_rays.y_pos.extend( vec![-1.0 * self.location[1];self.number_of_rays-1] );
                initial_rays.intensity.extend( vec![self.intensity;self.number_of_rays-1] );
                initial_rays.step_vector.extend( vec![dt;self.number_of_rays-1] );
                initial_rays.frequency.extend( vec![self.frequency;self.number_of_rays-1] ); */
                initial_rays.create_rays(initial_angles,
                    vec![self.location[0] ; self.number_of_rays],
                    vec![-1.0 * self.location[1];self.number_of_rays],
                    vec![self.intensity;self.number_of_rays],
                    vec![self.frequency;self.number_of_rays],
                    vec![dt;self.number_of_rays])
            }
            SourceType::Line => {
                println!("Not yet implemented");
            }
        } 
    }

}
