pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

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
            3=>velocity_silt(y, 0.1289E9),   //please change this to variable modulusoffrigidity 0.1289E9
            _=>300.0,
        }
    }
    else{
        velocity_silt(y, 0.1289E9)
    }
}



fn velocity_water(depth:f64) -> f64 {
    let salinity: f64=22.0;
    let latitude: f64=43.0;
    let temp: f64 = temperature_at_depth(depth);

    let speed: f64= 1402.5+5.0*temp-5.44e-2*temp*temp+2.1e-4*temp*temp+1.33*salinity-1.23e-2*salinity*temp+8.7e-5*salinity*temp*temp+1.56e-2*depth+2.55e-7*depth*depth-7.3e-12*depth*depth*depth+1.2e-6*depth*(latitude-45.0)-9.5e-13*temp*depth*depth*depth+3e-7*temp*temp*depth+1.43e-5*salinity*depth;
speed
}


fn velocity_silt( density: f64, modulusofrigidity: f64 ) -> f64 {
    let speedy = (modulusofrigidity/density).sqrt();
    speedy
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

