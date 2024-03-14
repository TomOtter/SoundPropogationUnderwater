pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

pub struct Ray {
        pub angle: f64,
        pub x_pos: f64,
        pub y_pos: f64,
        pub intensity: f64,
        pub stepVector: f64,


    } // Defines the properties of each ray.

    impl Ray {
        pub fn step(&mut self) {
            let new_x_pos = self.x_pos + self.stepVector * self.angle.tan();
            let new_y_pos = self.y_pos + self.stepVector;

            if material_speed(new_y_pos) > material_speed(self.y_pos) {
                let criticalAngle : f64 = (material_speed(self.y_pos)/material_speed(new_y_pos)).asin();
                if self.angle.abs() > criticalAngle.abs() {
                    self.angle = -1.0 * self.angle;
                    self.stepVector = self.stepVector * -1.0;
                }
                // Reflects the ray if its angle with the normal exceeds the critical angle.
            }

            let preangle = material_speed(new_y_pos)/material_speed(self.y_pos) * self.angle.sin();
            self.angle = preangle.asin();
            self.x_pos = new_x_pos;
            self.y_pos = new_y_pos;
            
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



fn material_speed(depth: f64) -> f64 {
    let y: f64 = depth;
    let result: f64;
    if y < 0.0 {
        result = 343.0;
    }
    else if y > 7000.0 {
        result = 4343.0;
    }
    else{ 
        result = (1521.45 - 0.0666*y + 0.0000343*y*y);
    } // Calculates the speed of sound in the respective material.
    result
}
