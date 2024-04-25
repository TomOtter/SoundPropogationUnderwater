use core::num;
use std::{
    fs::{self, File},
    io::prelude::*,
    path::Path,
    process::{Command, Output},
    time::Duration,
};
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
        let mut intensity_output: Vec<Vec<f64>> = Vec::with_capacity(size as usize);
        // Allocates the x and y ouputs. Inner vector indexes ray count, outer vector indexes time.

        let mut rays: Rays = Rays::initialise(number_of_rays);
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
            let intensity = rays.output_intensity();
            x_output.push( x_pos );
            y_output.push( y_pos );
            intensity_output.push( intensity );
        } // Time loop which pushes each ray by one step and outputs the new positions each iteration.
        self.output(x_output, y_output, intensity_output);
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

    fn output(&mut self, xdata: Vec<Vec<f64>>, ydata: Vec<Vec<f64>>, intensity_data: Vec<Vec<f64>>) {
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
                let intensity: f64 = intensity_data[i][j];
                output.push_str(&format!("{} {} {}\n", xpos, -1.0 * ypos, intensity));
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

//                                                  MARK: Rays Struct
pub struct Rays {
    angle: Vec<f64>,
    x_pos: Vec<f64>,
    y_pos: Vec<f64>,
    intensity: Vec<f64>,
    step_vector: Vec<f64>,
    frequency: Vec<f64>,
} // Defines the properties of each ray.

impl Rays {
    pub fn step(&mut self) {
        let mut new_x_pos: f64;
        let mut new_y_pos: f64;
        for i in 0..self.x_pos.len() {
            new_x_pos = self.x_pos[i] + self.step_vector[i] * material_speed(self.y_pos[i],self.x_pos[i]) * self.angle[i].sin();
            new_y_pos = self.y_pos[i] + self.step_vector[i] * material_speed(self.y_pos[i],self.x_pos[i]) * self.angle[i].cos();
            // Caluclates the new position of each ray after 1 time step

            if material_speed(new_y_pos, new_x_pos) > material_speed(self.y_pos[i], self.x_pos[i]) {
                let critical_angle : f64 = (material_speed(self.y_pos[i], self.x_pos[i])/material_speed(new_y_pos, new_x_pos)).asin();
                if self.angle[i].abs() > critical_angle.abs() {
                    self.angle[i] = -1.0 * self.angle[i];
                    self.step_vector[i] = self.step_vector[i] * -1.0;
                }
                // Reflects the ray if its angle with the normal exceeds the critical angle.
            }
            // Implement some if statement around here for reflection with boundary.

            let preangle = material_speed(new_y_pos, new_x_pos)/material_speed(self.y_pos[i], self.x_pos[i]) * self.angle[i].sin();
            self.x_pos[i] = new_x_pos;
            self.y_pos[i] = new_y_pos;
            self.angle[i] = preangle.asin();

            let salinity = 35.0;
            self.intensity[i] = 1.0 - calculate_absorption(self.frequency[i], temperature_at_depth(self.y_pos[i]), salinity, self.y_pos[i])
        }
    }


    pub fn bound_angles(&mut self, dt: f64) {
        for i in 0..self.angle.len() {

            if self.angle[i] > PI/2.0 {
                self.step_vector[i] = -1.0 * dt;
                self.angle[i] = -1.0 * (PI - self.angle[i])
            }
            else if self.angle[i] < -PI/2.0 {
                self.step_vector[i] = -1.0 * dt;
                self.angle[i] = -1.0 * (-PI - self.angle[i])
            }
            if self.angle[i] > 3.0 * PI/2.0 {
                self.step_vector[i] = dt;
                self.angle[i] = 1.0 * (3.0 * PI/2.0 - self.angle[i])
            }
            else if self.angle[i] < -3.0 * PI/2.0 {
                self.step_vector[i] = dt;
                self.angle[i] = 1.0 * (-3.0 * PI/2.0 - self.angle[i])
            }
        }
    } // Bounds the initial angle of the ray between +/- pi/2 rads (for maths purposes). Also converts the step to show downwards (-) or upwards (+) motion.
    
    pub fn initialise(number_of_rays: usize) -> Self {
        Self {
            angle: Vec::with_capacity(number_of_rays as usize),
            x_pos: Vec::with_capacity(number_of_rays as usize),
            y_pos: Vec::with_capacity(number_of_rays as usize),
            intensity: Vec::with_capacity(number_of_rays as usize),
            frequency: Vec::with_capacity(number_of_rays as usize),
            step_vector: Vec::with_capacity(number_of_rays as usize),
        }
    } // Initialisation function to define the initial size of the fields in Rays.

    pub fn create_rays(&mut self, angle: Vec<f64>, x_pos: Vec<f64>, y_pos: Vec<f64>,
         intensity: Vec<f64>, frequency: Vec<f64>, step_vector: Vec<f64>) -> () {
            self.angle.extend(angle);
            self.x_pos.extend(x_pos);
            self.y_pos.extend(y_pos);
            self.intensity.extend(intensity);
            self.frequency.extend(frequency);
            self.step_vector.extend(step_vector);
    } // Appends data of new rays to the vector fields under Rays.

    pub fn output_position(&self) -> (Vec<f64>, Vec<f64>) {
        (self.x_pos.clone(), self.y_pos.clone())
    } // Outputs a copy of each rays x and y position - to be used in functions implemented in other structs.

    pub fn output_intensity(&self) -> Vec<f64> {
        self.intensity.clone()
    } // Outputs a copy of each rays x and y position - to be used in functions implemented in other structs.
}


//                                             Ignore below for now

/* pub struct Boundary {
    x_limits : [f64;2],
    boundary_function : String,
    in_silt : bool,
}

impl Boundary {
    
    pub fn initialise(x_limits: [f64;2]) -> Self {
        if x_limits[0] >= x_limits[1] {
            eprintln!("Error: x_limits[0] must be less than x_limits[1].");
            std::process::exit(1);
        } // Ensures that the boundarys lower limit is less than its upper limit in the x-dimension
        Self {
            x_limits : x_limits,
            boundary_function : "y = 2000.0".to_string(),
            in_silt : false,
        }
    } // Initialisation function to define the fields inside of Boundary after undergoing necessary error checks.

    pub fn boundary_height(&mut self, x_pos: f64) -> f64 {
        let mut height: f64 = 0.0;
        if x_pos >= self.x_limits[0] && x_pos <= self.x_limits[1] {
            height = 2000.0;
        }      
        height
    } // Defines the height of the silt boundary at a given x position (convert to 'boundary_function' dependance later).

    pub fn material_speed(&mut self, y_pos: f64, x_pos: f64) -> f64 {
        let ycase: u32;
        let velocity_air: f64 = 343.0; // m s^-1

        if y_pos <= self.boundary_height(x_pos) {
            ycase = 3;
        } else if y_pos > 0.0 {
            ycase = 2
        } else {
            ycase = 1
        }

        match ycase{
            1=>velocity_water(y_pos),
            2=>velocity_air,
            3=>velocity_silt(y_pos, 0.1289E9),   //please change this to variable modulusoffrigidity 0.1289E9
            _=>300.0,
        }
    }
} */

//                                             MARK: Material Functions

pub fn material_speed(depth: f64, x: f64) -> f64 {
    let y: f64 = depth;
    let result: f64;
    let mut x_inside_boundary: bool;
    let mut y_inside_boundary: bool;
    let x_boundary: f64 = 1000.0;
    let y_boundary: f64 = 2000.0;
    let seasurface: f64 = 0.0;
    let mut v: f64; //local velocity
    let ycase: u32;
    let velocity_air: f64 = 343.0;
    if x.abs() < x_boundary {
        x_inside_boundary= true;
    }
    else {
        x_inside_boundary= false;
    }
 
    if y < y_boundary{ //checks above ocean floor
        if y > seasurface { // checks below sea surface
            ycase = 1;
            //under the water
        }
        else {
            ycase = 2; 
            //outside the water
        }
        
    }
    else {
        ycase = 3; //outside boundary
    }

    if x_inside_boundary == true { // & means and.
        //velocity_water(y)
        match ycase{
            1=>velocity_water(y),
            2=>velocity_air,
            3=>velocity_silt(y),   //please change this to variable modulusoffrigidity 0.1289E9
            _=>300.0,
        }
    }
    else{
        velocity_silt(y)
    }
}



fn velocity_water(depth:f64) -> f64 {
    let salinity: f64=22.0;
    let latitude: f64=43.0;
    let temp: f64 = temperature_at_depth(depth);

    let speed: f64= 1402.5+5.0*temp-5.44e-2*temp*temp+2.1e-4*temp*temp+1.33*salinity-1.23e-2*salinity*temp+8.7e-5*salinity*temp*temp+1.56e-2*depth+2.55e-7*depth*depth-7.3e-12*depth*depth*depth+1.2e-6*depth*(latitude-45.0)-9.5e-13*temp*depth*depth*depth+3e-7*temp*temp*depth+1.43e-5*salinity*depth;
speed
}



fn velocity_silt(depth:f64) -> f64 {
    let TurbiditeAreasVelocity:f64= (1.511+ 1.304*depth*0.001 - 0.257*depth*depth*depth*0.001*0.001*0.001)*1000.0;
    let SiliceousSedimentVelocity:f64 = (1.509 + 0.869*depth*0.001 - 0.267*depth*depth*0.001*0.001)*1000.0;
    let CalcerousSedimentsVelocity:f64 = (1.559 + 1.713*depth*0.001 - 0.374*depth*depth*0.001*0.001)*1000.0;
    let sandvelocity:f64=1626.0; 
TurbiditeAreasVelocity
}




fn temperature_at_depth(depth: f64) -> f64 {
    
    let surface_temp: f64 = 20.0;  // degrees C
    let bottom_temp:f64 = 4.0;  // degrees C
    let thermocline_start:f64 = 200.0;  // metres //temp is constant from 0-200m
    let thermocline_end:f64 = 1000.0;  // metres

  //constant temperature up to 200m based on literature - thermocline start depth
    if depth <= thermocline_start {
        surface_temp
    } else if depth >= thermocline_end {
        bottom_temp
    } else {
        // Linear interpolation within the thermocline
        let fraction = (depth - thermocline_start) / (thermocline_end - thermocline_start);
        surface_temp + fraction * (bottom_temp - surface_temp)
    }
}


// function to caculate absorption coefficient in seawater
fn calculate_absorption(f: f64, temp: f64, salinity: f64, ddepth: f64, ) -> f64 {

    let ph: f64  = 8.0;
   
    let z: f64 = 10.0; // just a constant

    let depth = ddepth / 1000.0;


    let f1 = 0.91 * (salinity / 35.0).sqrt() * (temp/33.0).exp();
    let f2 = 46.0 * (temp/18.0).exp();

    //Boric acid contribution
    let a = 0.101 * ((f1 * f.powi(2)) / (f1.powi(2) + f.powi(2))) * ((ph - 8.0) / 0.57).exp();
    //Magnesium Sulfate contribtion
    let b = 0.56 * (1.0 + (temp / 76.0)) * (salinity / 35.0) * ((f2 * f.powi(2)) / (f2.powi(2) + f.powi(2))) * (-depth / 4.9).exp();
    //Pure water contribution
    let c = (0.0004937-(2.59 *  z.powf(-5.0)) * temp + 9.11 * z.powf(-7.0) * temp.powi(2) -1.5010 * z.powf(-8.0) * temp.powi(3)) * ((1.0-((3.38 * z.powf(-2.0)) * depth) + (4.9 * z.powf(-4.0) * depth.powi(2))))* f.powi(2);
    //let c = 0.00049 * f.powi(2) * e.powf(-(temp / 27.0 + depth / 17.0));



    (a + b + c) / 1000.0 //dB/m


    
}

