pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

pub struct Ray {
        pub angle: f64,
        pub x_pos: f64,
        pub y_pos: f64,
        pub intensity: f64,
        pub stepVector: f64,
        pub frequency: f64,


    } // Defines the properties of each ray.

    impl Ray {
        pub fn step(&mut self) {
            let new_x_pos = self.x_pos + self.stepVector * self.angle.tan();
            let new_y_pos = self.y_pos + self.stepVector;

            if material_speed(new_y_pos, new_x_pos) > material_speed(self.y_pos, self.x_pos) {
                let criticalAngle : f64 = (material_speed(self.y_pos, self.x_pos)/material_speed(new_y_pos, new_x_pos)).asin();
                if self.angle.abs() > criticalAngle.abs() {
                    self.angle = -1.0 * self.angle;
                    self.stepVector = self.stepVector * -1.0;
                }
                // Reflects the ray if its angle with the normal exceeds the critical angle.
            }

            let preangle = material_speed(new_y_pos, new_x_pos)/material_speed(self.y_pos, self.x_pos) * self.angle.sin();
            self.angle = preangle.asin();
            self.x_pos = new_x_pos;
            self.y_pos = new_y_pos;

            let salinity = 35.0;
            self.intensity = 1.0 - calculate_absorption(self.frequency, temperature_at_depth(self.y_pos), salinity, self.y_pos)
            
        } // Calculates the new angle and position of the ray after one step is taken.


        pub fn initialise(&mut self, dy: f64) {

            let initialAngle = self.angle;

            let mut angle: f64 = initialAngle;
            let mut stepVector: f64 = dy;

            if initialAngle > PI/2.0 {
                stepVector = -1.0 * dy;
                angle = -1.0 * (PI - initialAngle)
            }
            else if initialAngle < -PI/2.0 {
                stepVector = -1.0 * dy;
                angle = -1.0 * (-PI - initialAngle)
            }
            if initialAngle > 3.0 * PI/2.0 {
                stepVector = dy;
                angle = 1.0 * (3.0 * PI/2.0 - initialAngle)
            }
            else if initialAngle < -3.0 * PI/2.0 {
                stepVector = dy;
                angle = 1.0 * (-3.0 * PI/2.0 - initialAngle)
            }

            self.stepVector = stepVector;
            self.angle = angle;
        } // Bounds the initial angle of the ray between +/- pi/2 rads (for maths purposes). Also converts the step to show downwards (-) or upwards (+) motion.
    }

//-------------------------------------------------------------------------------------------------
//----------------------------------------------------------------------------------------------------





fn material_speed(depth: f64, x: f64) -> f64 {
    let y: f64 = depth;
    let result: f64;
    let mut xInsideBoundary: bool;
    let mut yInsideBoundary: bool;
    let xBoundary: f64 = 1000.0;
    let yBoundary: f64 = 4000.0;
    let seasurface: f64 = 0.0;
    let mut v: f64; //local velocity
    let mut ycase: u32;
    let velocityAir: f64 = 343.0;
    if x.abs() < xBoundary {
        xInsideBoundary= true;
    }
    else {
        xInsideBoundary= false;
    }
 
    if y < yBoundary{ //checks above ocean floor
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

    if xInsideBoundary == true { // & means and.
        //velocityWater(y)
        match ycase{
            1=>velocityWater(y),
            2=>velocityAir,
            3=>velocitySilt(y, 0.1289E9),   //please change this to variable modulusoffrigidity 0.1289E9
            _=>300.0,
        }
    }
    else{
        velocitySilt(y, 0.1289E9)
    }
}





//------------------------------------------------------------------------

fn velocityWater(depth:f64) -> f64 {
    let salinity: f64=22.0;
    let latitude: f64=43.0;
    let temp: f64 = temperature_at_depth(depth);

    let speed: f64= 1402.5+5.0*temp-5.44e-2*temp*temp+2.1e-4*temp*temp+1.33*salinity-1.23e-2*salinity*temp+8.7e-5*salinity*temp*temp+1.56e-2*depth+2.55e-7*depth*depth-7.3e-12*depth*depth*depth+1.2e-6*depth*(latitude-45.0)-9.5e-13*temp*depth*depth*depth+3e-7*temp*temp*depth+1.43e-5*salinity*depth;
speed
}


fn velocitySilt( density: f64, modulusofrigidity: f64 ) -> f64 {
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
   
    let z: f64 = 10.0; // just a contant

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

