use std::{any::type_name, cmp::Ordering, collections::HashMap, fmt::format, fs::{self, File}, io::prelude::*, path::Path, process::{Command, Output}
};
use crate::material::{self, Material, MaterialType};

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;
pub enum SourceType {
    Point,
    Line,
}

//                                               MARK: Simulation Struct
pub struct Simulation<F: SingleInputFunction> {
    sources : Vec<Source>,
    grid: Grid,
    rays: Rays,
    boundaries: Vec<Boundary<F>>,
}

impl<F: SingleInputFunction> Simulation<F> {

    pub fn new(square_size: f64, simulation_x_range: [f64;2], simulation_y_range: [f64;2]) -> Self {
        let grid = Grid::initialise(square_size, simulation_x_range, simulation_y_range);
        Self {
            sources : Vec::new(),
            grid : grid,
            rays : Default::default(),
            boundaries : Vec::new(),
            // Defines all other 'child' structs under the parent. 'rays' has not yet been defined.
        }
    } // Initialisation function to define the fields inside of Simulation after undergoing necessary error checks.

    pub fn add_source(&mut self, start_angle: f64, end_angle: f64, number_of_rays: i32, intensity: f64, frequency: f64, location: [f64;2], source_type: SourceType) -> () {
        if start_angle.abs() > PI || end_angle.abs() > PI {
            eprintln!("Error: Minimum and maximum angles must be within the range of -π to π.");
            std::process::exit(1);
        } // Checks if the minimum and maximum angles are within the range of +/- PI.

        let new_source: Source = Source::initialise(source_type, start_angle, end_angle, 
            number_of_rays as usize, intensity, frequency, location);
        
        self.sources.push(new_source);
        // Adds new source to an array of sources under the Simulation struct.
    }

    pub fn add_boundary(&mut self, material: MaterialType, shape_function: F) -> ()
    where
        F: SingleInputFunction + 'static,
    {
        let new_boundary = Boundary::initialise(Box::new(shape_function), material);
        self.boundaries.push( new_boundary.unwrap() );
    }

    pub fn x_limits(&mut self, limits: [f64;2]) -> () {
        if let Some(last_boundary) = self.boundaries.last_mut() {
            last_boundary.set_x_limits(limits);
        }
    }

    pub fn y_upper_limit(&mut self, limit: f64) -> () {
        if let Some(last_boundary) = self.boundaries.last_mut() {
            last_boundary.set_y_maximum(limit);
        }
    }

    fn calculate(&mut self, dt: f64, duration: f64, frames: i32) -> f64 {
        if self.sources.len() == 0 {
            eprintln!("Error: No sources have been defined. Call 'self.addSource' prior to this function to define a soundwave source.");
            std::process::exit(1);
        } // Ensures that a source has been defined prior to this function.
        
        self.create_folder("./outputdata");
        // Creates folder for data files to be stored

        let size: i32 = (duration / dt) as i32;
        let frame_spacing: i32 = size / frames;
        let mut max_init_intensity = 0.0;
        let number_of_rays: usize = self.sources.iter().map(|source| source.number_of_rays as usize).sum();
        //Sums 'number_of_rays' across all sources.

        self.rays = Rays::initialise(number_of_rays);
        // Defines the Rays struct with each variable inside having an appendable vector with minimum array size (beneficial for memory).
        
        for i in 0..self.sources.len() {
            self.sources[i].create_rays(&mut self.rays);
            if self.sources[i].intensity / (self.sources[i].number_of_rays as f64).powi(2) > max_init_intensity {
                max_init_intensity = self.sources[i].intensity / (self.sources[i].number_of_rays as f64).powi(2);
            }
        } // Compiles all of the initial data for each ray, from its sources, into one 'Rays' struct.

        self.rays.bound_angles([0,self.rays.x_pos.len()]);

        for i in 0..size {
            if i != 0{
                self.grid.squares.clear();
                self.rays.step(dt, &mut self.boundaries, self.grid.x_range, self.grid.y_range, max_init_intensity);  
            } // Done to ensure that the initial positions of the rays is not overwritten in the output file.
            if (i % frame_spacing) == 0 {
                for j in 0..self.rays.x_pos.len() {
                    let phase = self.rays.output_phase(j as usize);
                    self.grid.append([self.rays.x_pos[j], self.rays.y_pos[j]], self.rays.intensity[j], phase);
                    // Adds the intensity and phase shift to a specific 'grid square' (location defined by ray position).
                }
                let (xpos, ypos, intensity) = self.grid.output_data();
                self.output(xpos, ypos, Some(intensity), format!("/dataset{}", i / frame_spacing));
                // Outputs the intensitys at each grid square to a file
            }
        } // Time loop which pushes each ray by one step and outputs the new positions each iteration.

        max_init_intensity
    }

//                                                    MARK: Outputs

    fn create_folder(&mut self, folder_path: &str) {
        let path = Path::new(folder_path);

        // Check if the provided path is absolute
        if path.is_absolute() {
            eprintln!("Error: Absolute paths are not allowed.");
            std::process::exit(1);
        }

        // Close and delete the directory
        if path.exists() {
            drop(fs::read_dir(folder_path));
            match fs::remove_dir_all(folder_path) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error deleting directory {}: {}", folder_path, err);
                    std::process::exit(1);
                }
            }
        }
        // Create the new directory
        match fs::create_dir(folder_path) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error creating directory {}: {}", folder_path, err);
                std::process::exit(1);
            }
        }
    }

    fn output(&mut self, xpos: Vec<f64>, ypos: Vec<f64>, additional_data: Option<Vec<f64>>, filename: String) -> () {
            let mut output = String::new();
            // Create a string to hold the output for this iteration

            // Append position data to the output string
            if let Some(intensity) = additional_data {
                for i in 0..xpos.len() { output.push_str(&format!("{} {} {}\n", xpos[i], ypos[i], intensity[i])) }
            } else { for i in 0..xpos.len() { output.push_str(&format!("{} {}\n", xpos[i], ypos[i])) } }

            let folder_path = "./outputdata";
            // Define the folder path where output files will be stored

            let file_name = format!("{}{}.txt", folder_path, filename);
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
    }

    pub fn generate_gif(&mut self, duration: f64, dt: f64, frames: i32) -> Output {
        if frames as f64 > (duration / dt) {
            eprintln!("Error: There is not enough time steps to accomodate the requested number of frames. Consider decreasing dt or frames.");
            std::process::exit(1);
        } // Terminates the program if the number of frames requested is greater than the maximum possible number of files produced

        let max_intensity = self.calculate(dt, duration, frames);
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

        // Adds the data for each boundary for the GIF
        for i in 0..self.boundaries.len() {
            let mut boundary_x = vec![0.0 ; 1000];
            let mut boundary_y: Vec<f64> = vec![0.0 ; 1000];
            let mut index : usize = 0;
            for j in 1..1001 {
                boundary_x[index] = j as f64 * (self.grid.x_range[1] - self.grid.x_range[0]) / 1000.0 + self.grid.x_range[0];
                if let Some(height) = self.boundaries[i].boundary_height(boundary_x[index]) {
                    if height.is_nan() {
                        boundary_y[index] = self.grid.y_range[0];
                    } else if height.is_infinite() {
                        boundary_y[index] = self.grid.y_range[1];
                    } else {
                        boundary_y[index] = height;
                    }
                    index += 1;
                } else {
                    if self.boundaries[i].boundary_height( (j as f64 - 1.0) * (self.grid.x_range[1] - self.grid.x_range[0]) / 1000.0 + self.grid.x_range[0] ) != None {
                        boundary_x.remove(index);
                        boundary_y.remove(index);
                    } else { 

                        boundary_y[index] = self.grid.y_range[0];
                        index += 1;
                    }
                }
            }
            if self.boundaries[i].boundary_height(self.grid.x_range[0]) == None {
                boundary_x.insert(0, boundary_x[0] - (self.grid.x_range[1] - self.grid.x_range[0]) / 1000.0);
                boundary_y.insert(0, self.grid.y_range[0]);
            }
            self.output(boundary_x, boundary_y, None, format!("/boundary{}", i));
        }
        
        self.create_folder("./outputImages");

        let length = txt_files.len();
        let cmd = format!("runGifMAker.bat {} {} {} {} {} {} {} {}",
         length, self.boundaries.len(), self.grid.x_range[0], self.grid.x_range[1], self.grid.y_range[0], self.grid.y_range[1], duration, max_intensity);

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
    pub fn initialise(source_type: SourceType, start_angle: f64, mut end_angle: f64,
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

        if end_angle <= start_angle {
            end_angle += 2.0 * PI;
        }

        Self {
            source_type : source_type,
            angle_range : [start_angle, end_angle],
            number_of_rays : number_of_rays,
            intensity : intensity,
            frequency : frequency,
            location : location,
        }
    } // Initialisation function to define the fields inside of Struct after undergoing necessary error checks.

    fn create_rays(&mut self, initial_rays: &mut Rays) {
        let mut initial_angles: Vec<f64> = Vec::with_capacity(self.number_of_rays);
        match self.source_type {
            SourceType::Point => {
                let angle_spacing: f64 = (self.angle_range[1] - self.angle_range[0]) / (self.number_of_rays as f64);

                for i in 0..self.number_of_rays {
                    let mut ray_angle: f64 = self.angle_range[0] + (angle_spacing * i as f64);
                    if ray_angle > (2.0 * PI) { ray_angle -= 2.0 * PI }
                    initial_angles.push( ray_angle );
                } // Evenly spaces out the arrays at the source between the given bounds and appends to the initial ray angles struct.

                let local_ray_intensity: f64 = self.intensity / f64::powi(self.number_of_rays as f64, 2);

                initial_rays.create_rays(initial_angles,
                    vec![self.location[0] ; self.number_of_rays],
                    vec![-1.0 * self.location[1];self.number_of_rays],
                    vec![local_ray_intensity;self.number_of_rays],
                    vec![self.frequency;self.number_of_rays],
                    vec![1.0;self.number_of_rays])
            }
            SourceType::Line => {
                println!("Not yet implemented");
            }
        } 
    }

}

//                                                  MARK: Rays Struct
#[derive(Default)]
pub struct Rays {
    angle: Vec<f64>,
    x_pos: Vec<f64>,
    y_pos: Vec<f64>,
    intensity: Vec<f64>,
    step_vector: Vec<f64>,
    frequency: Vec<f64>,
    propagation_time: Vec<f64>
} // Defines the properties of each ray.

impl Rays {
    pub fn initialise(number_of_rays: usize) -> Self {
        Self {
            angle: Vec::with_capacity(number_of_rays as usize),
            x_pos: Vec::with_capacity(number_of_rays as usize),
            y_pos: Vec::with_capacity(number_of_rays as usize),
            intensity: Vec::with_capacity(number_of_rays as usize),
            frequency: Vec::with_capacity(number_of_rays as usize),
            step_vector: Vec::with_capacity(number_of_rays as usize),
            propagation_time: Vec::with_capacity(number_of_rays as usize),
        }
    } // Initialisation function to define the initial size of the fields in Rays.
    
    fn bound_angles(&mut self, index_range: [usize;2]) {
        for i in index_range[0]..index_range[1] {

            if self.angle[i] > PI/2.0 {
                self.step_vector[i] = -1.0;
                self.angle[i] = -1.0 * (PI - self.angle[i])
            }
            else if self.angle[i] < -PI/2.0 {
                self.step_vector[i] = -1.0;
                self.angle[i] = -1.0 * (-PI - self.angle[i])
            }
            if self.angle[i] > 3.0 * PI/2.0 {
                self.step_vector[i] = 1.0;
                self.angle[i] = 1.0 * (3.0 * PI/2.0 - self.angle[i])
            }
            else if self.angle[i] < -3.0 * PI/2.0 {
                self.step_vector[i] = 1.0;
                self.angle[i] = 1.0 * (-3.0 * PI/2.0 - self.angle[i])
            }
        }
    } // Bounds the initial angle of the ray between +/- pi/2 rads (for maths purposes). Also converts the step to show downwards (-) or upwards (+) motion.

    fn create_rays(&mut self, angle: Vec<f64>, x_pos: Vec<f64>, y_pos: Vec<f64>,
         intensity: Vec<f64>, frequency: Vec<f64>, step_vector: Vec<f64>) -> () {
            self.angle.extend(&angle);
            self.x_pos.extend(x_pos);
            self.y_pos.extend(y_pos);
            self.intensity.extend(intensity);
            self.frequency.extend(frequency);
            self.step_vector.extend(step_vector);
            self.propagation_time.extend( vec![0.0;angle.len()] );
    } // Appends data of new rays to the vector fields under Rays.

    fn step<F: SingleInputFunction>(&mut self, dt: f64, boundaries: &mut Vec<Boundary<F>>, simulation_x_limit: [f64;2], simulation_y_limit: [f64;2], init_max_intensity: f64) -> () {
        let mut new_x_pos: f64;
        let mut new_y_pos: f64;
        let mut i: usize = 0;

        while i != self.x_pos.len() {
            // Removes data if it leaves the simulation range
            if (self.x_pos[i] < simulation_x_limit[0]) || (self.x_pos[i] > simulation_x_limit[1]) || (-self.y_pos[i] < simulation_y_limit[0]) ||
                 (-self.y_pos[i] > simulation_y_limit[1]) || (self.intensity[i].is_finite() == false) || (self.intensity[i] < init_max_intensity / 10000000000.0) {
                self.angle.remove(i);
                self.x_pos.remove(i);
                self.y_pos.remove(i);
                self.intensity.remove(i);
                self.step_vector.remove(i);
                self.frequency.remove(i);
                self.propagation_time.remove(i);
            } else { 
                let (old_ray_speed, old_boundary) = self.ray_speed(self.x_pos[i],self.y_pos[i], boundaries);

                // Caluclates the new position of each ray after 1 time step
                self.propagation_time[i] += dt;
                new_x_pos = self.x_pos[i] + self.step_vector[i] * dt * old_ray_speed * self.angle[i].sin();
                new_y_pos = self.y_pos[i] + self.step_vector[i] * dt * old_ray_speed * self.angle[i].cos();

                let (new_ray_speed, new_boundary) = self.ray_speed(new_x_pos, new_y_pos, boundaries);

                let material_change_test = match (&old_boundary, &new_boundary) {
                    (Some(location_1), Some(location_2)) => {
                        if location_1.material != location_2.material { 1 }
                        else { 0 }
                    }
                    (Some(_), None) => 2,
                    (None, Some(_)) => 1,
                    _ => 0
                };

                if material_change_test != 0 {
                    let r_coeff: f64;
                    let t_coeff: f64;
                    if material_change_test == 1 { 
                        self.reflection(new_boundary.clone().unwrap(), new_x_pos, new_y_pos, i); 
                        (r_coeff, t_coeff) = self.reflection_and_transmission(old_boundary, new_boundary,
                        old_ray_speed, new_ray_speed, i);
                    } else { 
                        self.reflection(old_boundary.clone().unwrap(), new_x_pos, new_y_pos, i);
                        (r_coeff, t_coeff) = self.reflection_and_transmission(old_boundary, new_boundary,
                        old_ray_speed, new_ray_speed, i);
                    }
                    *self.intensity.last_mut().unwrap() *= r_coeff;
                    self.intensity[i] *= t_coeff;
                }

                
                if new_ray_speed > old_ray_speed {
                    let critical_angle : f64 = (old_ray_speed/new_ray_speed).asin();
                    // Reflects the ray if its angle with the normal exceeds the critical angle.
                    if self.angle[i].abs() > critical_angle.abs() {
                        self.angle[i] = -1.0 * self.angle[i];
                        self.step_vector[i] = self.step_vector[i] * -1.0;
                    }
                }

                self.x_pos[i] = new_x_pos;
                self.y_pos[i] = new_y_pos;
                self.angle[i] = ( new_ray_speed / old_ray_speed * self.angle[i].sin() ).asin();

                let salinity = 35.0;
                let temperature = self.temperature_at_depth(self.y_pos[i]);
                self.intensity[i] *= 1.0 - self.calculate_absorption(self.frequency[i], temperature, salinity, self.y_pos[i]);
                i += 1;
            }
        }
    }

    fn output_phase(&self, index: usize) -> f64 {
        2.0 * PI * self.frequency[index] * self.propagation_time[index]
    }

    fn ray_speed<F: SingleInputFunction>(&mut self, x_pos: f64, y_pos: f64, boundaries: &mut Vec<Boundary<F>>) -> (f64, Option<Boundary<F>>) {
        let mut current_boundary: Option<usize> = None;
        let mut boundary_height: Option<f64> = None;
        let velocity_air: f64 = 343.0; // m s^-1
        let ycase: u32;

        // Filters out any None variables
        let mut valid_boundaries: Vec<_> = boundaries
            .iter_mut()
            .filter(|b| {
                if let Some(height) = b.boundary_height(x_pos) {
                    // Filter out NaN and infinite values
                    height.is_finite()
                } else {
                    // If the height is None, consider it invalid
                    false
                }
            })
            .collect();

        // For the cases where the is a boundary at some y position at the given x position
        if !valid_boundaries.is_empty() {
            
            // Sorts valid_boundaries in order of magnitude of the output of boundary_height.unwrap()
            valid_boundaries.sort_by(|a, b| {
                let a_height = a.boundary_height(x_pos).unwrap_or_default();
                let b_height = b.boundary_height(x_pos).unwrap_or_default();
                a_height.partial_cmp(&b_height).unwrap()
            });

            // Checks if the ray is inside any boundary
            if -y_pos < valid_boundaries[valid_boundaries.len()-1].boundary_height(x_pos).unwrap() {
                ycase = 3; // Boundary

                // Determines the boundary that the ray is in
                for i in 0..valid_boundaries.len() {
                    if let Some(height) = valid_boundaries[i].boundary_height(x_pos) {
                        if height > -y_pos && i != valid_boundaries.len() {
                            current_boundary = Some(i);
                            boundary_height = Some(height);
                            break;
                        } else if i == valid_boundaries.len() { 
                            current_boundary = Some(valid_boundaries.len());
                            boundary_height = valid_boundaries[current_boundary.unwrap()].boundary_height(x_pos);
                        }
                    }
                }
            } 
            else if -y_pos > 0.0 { ycase = 2; } // Air
            else { ycase = 1; } // Water

        } else {
            if -y_pos > 0.0 { ycase = 2; } // Air
            else { ycase = 1; } // Water
        }

        match ycase{
            1=>(self.velocity_water(-y_pos), None),
            2=>(velocity_air, None),
            _=>(valid_boundaries[current_boundary.unwrap()].material.calculate_velocity(-y_pos - boundary_height.unwrap()), 
                Some( valid_boundaries[current_boundary.unwrap()].clone() )),
        }
    }

    fn reflection<F: SingleInputFunction>(&mut self, mut boundary: Boundary<F>, new_x_pos: f64, new_y_pos: f64, ray_index: usize) -> () {
        let delta_x = new_x_pos - self.x_pos[ray_index];
        let delta_y = new_y_pos - self.y_pos[ray_index];
        let slope = boundary.differentiate(new_x_pos);
        let normal = -slope.powi(-1);
        let reflected_angle: f64;
        let step_vector: f64;

        const TOLERANCE: f64 = 0.75;

        if normal.is_infinite() { 
            reflected_angle = - self.angle[ray_index];
            step_vector = - self.step_vector[ray_index]
        } else if normal.is_nan() {
            reflected_angle = self.angle[ray_index];
            step_vector = - self.step_vector[ray_index]
        } else {
            let dot_product = delta_x + delta_y * normal;

            let new_delta_x = delta_x - 2.0 * dot_product;
            let new_delta_y = delta_y - 2.0 * dot_product * normal;

            reflected_angle = ( new_delta_x / new_delta_y ).atan();
            step_vector = self.step_vector[ray_index]
        }

        if (reflected_angle - slope.atan()).abs() > TOLERANCE {
            self.create_rays(vec![reflected_angle], vec![self.x_pos[ray_index]], vec![self.y_pos[ray_index]],
                vec![self.intensity[ray_index]], vec![self.frequency[ray_index]], vec![step_vector]);

            self.bound_angles([self.x_pos.len(), self.x_pos.len()]);
        }
    }

    fn reflection_and_transmission<F: SingleInputFunction>(&mut self, material_1: Option<Boundary<F>>, material_2: Option<Boundary<F>>, old_speed: f64, new_speed: f64, ray_index: usize) -> (f64, f64) {
        let z1: f64;
        let z2: f64;
        if let Some(boundary_1) = material_1 { 
            z1 = self.acoustic_impedance(ImpedenceInput::Material(boundary_1.material),old_speed, boundary_1.boundary_height(self.x_pos[ray_index]), ray_index);
        }
        else {
            let density: f64;
            if self.y_pos[ray_index] > 0.0 { density = 1.293 }
            else { density = self.density_water(ray_index) }
            z1 = self.acoustic_impedance(ImpedenceInput::Density(density), old_speed, None, ray_index);
        }
        if let Some(boundary_2) = material_2 { 
            z2 = self.acoustic_impedance(ImpedenceInput::Material(boundary_2.material),new_speed, boundary_2.boundary_height(self.x_pos[ray_index]), ray_index); 
        }
        else {
            let density: f64;
            if self.y_pos[ray_index] > 0.0 { density = 1.293 }
            else { density = self.density_water(ray_index) }
            z2 = self.acoustic_impedance(ImpedenceInput::Density(density), new_speed, None, ray_index);
        }
    
        let r_coeff = ((z2 * (self.angle[ray_index]).cos()) - (z1 * (self.angle[ray_index]).cos())) / ((z2 * (self.angle[ray_index]).cos()) + (z1*(self.angle[ray_index]).cos()));
        let t_coeff = 1.0 - r_coeff;
    
        (r_coeff, t_coeff)
    }

    fn acoustic_impedance(&mut self, input: ImpedenceInput, speed_of_sound: f64, boundary_height: Option<f64>, ray_index: usize) -> f64 {
        match input {
            ImpedenceInput::Material(material) => material.clone().acoustic_impedance(speed_of_sound, self.y_pos[ray_index], boundary_height.unwrap()),
            ImpedenceInput::Density(density) => density * speed_of_sound
        }
    }

    //                                                  MARK: Water Properties

    fn velocity_water(&mut self, depth:f64) -> f64 {
        let salinity: f64=22.0;
        let latitude: f64=43.0;
        let temp: f64 = self.temperature_at_depth(depth);
    
        let speed: f64= 1402.5+5.0*temp-5.44e-2*temp.powi(2)+2.1e-4*temp.powi(2)+1.33*salinity-1.23e-2*salinity*temp+8.7e-5*salinity*temp.powi(2)+1.56e-2*depth+2.55e-7*depth.powi(2)-7.3e-12*depth.powi(3)+1.2e-6*depth*(latitude-45.0)-9.5e-13*temp*depth.powi(3)+3e-7*temp.powi(2)*depth+1.43e-5*salinity*depth;
        speed
    }

    fn density_water(&mut self, ray_index: usize) -> f64 {
        let temperature = self.temperature_at_depth(self.y_pos[ray_index]);
        999.974950 * (1.0 - (temperature - 3.983035).powi(2) * (temperature + 301.797) / (522528.9) * (temperature + 69.34881) )
    }

    fn temperature_at_depth(&mut self, depth: f64) -> f64 {
    
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
    fn calculate_absorption(&mut self, f: f64, temp: f64, salinity: f64, ddepth: f64, ) -> f64 {
    
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
}

enum ImpedenceInput {
    Material(Material),
    Density(f64),
}

//                                                  MARK: Boundary Struct

pub trait SingleInputFunction: Clone {
    fn evaluate(&self, x: f64) -> f64;
}

impl<F> SingleInputFunction for F
where
    F: Fn(f64) -> f64 + Clone,
{
    fn evaluate(&self, x: f64) -> f64 {
        self(x)
    }
}

pub struct Boundary<F: SingleInputFunction> {
    shape_function : Box<F>,
    x_limits : [Option<f64>;2],
    y_maximum : Option<f64>,
    current_y : Option<f64>,
    material : Material,
}

impl<F: SingleInputFunction> Boundary<F> {
    pub fn initialise(shape_function: Box<F>, material: MaterialType) -> Result<Self, &'static str>
    where
        F: SingleInputFunction + 'static,
    {
        let material_properites = Material::define(material);

        Ok(Boundary{
            shape_function: shape_function,
            x_limits: [None, None],
            y_maximum: Some(0.0),
            current_y: None,
            material: material_properites,
        })
    }

    pub fn set_x_limits(&mut self, limits: [f64;2]) -> () {
        self.x_limits = [Some(limits[0]), Some(limits[1])];
    }

    pub fn set_y_maximum(&mut self, limit: f64) -> () {
        self.y_maximum = Some(limit);
    }

    fn boundary_height(&self, x:f64) -> Option<f64> {
        if let [Some(x_min), Some(x_max)] = self.x_limits {
            if x < x_min || x > x_max {
                return None;
            }
        }

        let mut y_boundary = self.shape_function.evaluate(x);

        if let Some(y_max) = self.y_maximum {
            if y_boundary > y_max {
                y_boundary = y_max;
            }
        }

        Some(y_boundary)
    }

    pub fn differentiate(&mut self, x_pos: f64) -> f64 {
        let h = 0.0000001;
        let mut result: f64 = f64::NAN;
        if let Some(next_height) = self.boundary_height( x_pos + h / 2.0 ) {
            if let Some(prior_height) = self.boundary_height( x_pos - h / 2.0 ) {
                result = (next_height - prior_height) / h;
            } else {
                result = (next_height - self.boundary_height(x_pos).unwrap()) / h;
            }
        } else if let Some(prior_height) = self.boundary_height( x_pos - h / 2.0 ) {
            result = (self.boundary_height(x_pos).unwrap() - prior_height) / h;
        }
        result
    } 
 
}

impl<F: SingleInputFunction + Clone> Clone for Boundary<F> {
    fn clone(&self) -> Self {
        Boundary {
            shape_function: self.shape_function.clone(),
            x_limits: self.x_limits,
            y_maximum: self.y_maximum,
            current_y: self.current_y,
            material: self.material,
        }
    }
}

//                                                  MARK: Grid Struct

pub struct Grid {
    squares: HashMap< (usize, usize), Option< (Vec<f64>, Vec<f64>) > >,
    x_range: [f64;2],
    y_range: [f64;2],
    square_size: f64,
}

impl Grid {

    pub fn initialise(square_size: f64, simulation_x_range: [f64;2], simulation_y_range: [f64;2]) -> Self {
        if square_size <= 0.0 {
            eprintln!("Error: square_size must be a positive, non-zero, integer.");
            std::process::exit(1);
        }
        if simulation_x_range[0] >= simulation_x_range[1] {
            eprintln!("Error: simulation_x_range[0] must be less than simulation_x_range[1].");
            std::process::exit(1);
        }
        if simulation_y_range[0] >= simulation_y_range[1] {
            eprintln!("Error: simulation_y_range[0] must be less than simulation_y_range[1].");
            std::process::exit(1);
        }
        Self {
            squares: HashMap::new(),
            x_range: simulation_x_range,
            y_range: simulation_y_range,
            square_size: square_size,
        }
    }

    fn append(&mut self, location: [f64; 2], intensity: f64, phase_shift: f64) -> () {
        let x_grid: usize = ( (location[0] - self.x_range[0] ) / self.square_size) as usize;
        let y_grid: usize = ( (location[1] - self.y_range[0] ) / self.square_size) as usize;
        // Converts the position of the ray into a grid coordinate

        if let Some(existing_intensity) = self.squares.get_mut(&(x_grid, y_grid)) {
            // Get a mutable reference to the intensity value in the grid square
            match existing_intensity {
                Some((intensity_ref, phase_ref)) => {
                    intensity_ref.push(intensity);
                    phase_ref.push(phase_shift);
                } // If the intensity and phase value exists, append the intensity and phase difference
                None => {
                    *existing_intensity = Some( (vec![intensity], vec![phase_shift]) );
                } // If the intensity value doesn't exist, insert the new intensity and phase difference
            }
        } else {
            self.squares.insert((x_grid, y_grid), Some( (vec![intensity], vec![phase_shift]) ));
        } // If the grid square doesn't exist, create it and insert the new intensity and phase shift
    }

    fn output_data(&self) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut x_positions = Vec::new();
        let mut y_positions = Vec::new();
        let mut intensities = Vec::new();
    
        for ((x, y), ray_data) in &self.squares {
            let mut superimposed_intensity: f64 = 0.0;

            if let Some((grid_intensities, grid_phases)) = ray_data {
                // Check if the grid square contains intensity values

                for i in 0..grid_intensities.len() {
                    if grid_intensities.len() != 1 {
                        for j in (i+1)..grid_intensities.len() {
                                superimposed_intensity += 2.0 * f64::sqrt(grid_intensities[i] * grid_intensities[j]) * (grid_phases[i] - grid_phases[j]).cos()
                        }
                    }
                    superimposed_intensity += grid_intensities[i];
                } // I_eff = Sum over all i=1...N (I_i) + 2 * Sum over all i,j>i ( sqrt(I_i * I_j) cos(phi_i - phi_j))

                x_positions.push( (*x as f64 + 0.5) * self.square_size + self.x_range[0]);
                y_positions.push( -1.0 * ((*y as f64 + 0.5) * self.square_size + self.y_range[0]) );
                intensities.push(superimposed_intensity);
                // Appends data to output. Position data is converted to output the centre of its grid square.
            }
        }
        (x_positions, y_positions, intensities)
        // Return a tuple containing the vectors of x positions, y positions, intensities
    }
}

