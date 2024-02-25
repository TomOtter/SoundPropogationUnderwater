use std::fs::File;
use std::io::prelude::*;


mod ray_trace {
    pub struct Ray {
        pub angle: f64,
        pub x_pos: f64,
        pub y_pos: f64,
        pub intensity: f64,
        pub stepVector: f64,


    }

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
                // Reflects the ray if its angle with the normal exceeds the critical angle
            }

            let preangle = material_speed(new_y_pos)/material_speed(self.y_pos) * self.angle.sin();
            self.angle = preangle.asin();
            self.x_pos = new_x_pos;
            self.y_pos = new_y_pos;
            
        }
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
        }
        result
    }
}


pub const PI: f64 = 3.14159265358979323846264338327950288_f64;




// Determines the speed of the ray depepndant on the depth 'z'.
// z > 0 -> air,   0 > z > 4000 -> water,   z < 4000 -> sea floor 



fn calcRayPath(initialAngle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_directions: [f64;SIZE] = [0.0;SIZE];

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
    // Bounds the angle of the ray between +/- pi/2 from the normal and flips the direction of the step vector.

    ray_ypositions[0] = 1200.0;
    ray_directions[0] = angle;

    // Sets the starting position and angle of each ray

    let mut ray1 = ray_trace::Ray {
        angle: angle,
        x_pos: ray_xpositions[0],
        y_pos: ray_ypositions[0],
        intensity: 1.0,
        stepVector: stepVector,
    };


    for i in 0..SIZE-1 {

        ray1.step();

        ray_xpositions[i+1] = ray1.x_pos;
        ray_ypositions[i+1] = ray1.y_pos;
        //Calculates the new position of the ray after a step is taken
    }

    (ray_xpositions,ray_ypositions)
    // Outputs the x-y positions of the ray for each iterative step
}

const SIZE: usize = 200;

fn main() -> std::io::Result<()> {
    

    let dy: f64 = 10.0;


    //Sets step size
    
    let mut output : String = "\n".to_string();
    //-120..121
    for i in -120..121{
        let angle = ((i) as f64)  * PI/120.00 ;
        // Sets the initial angle of the ray. The initial angle of each ray increases by pi/120 in each iteration between +/- pi.

        if angle.sin().abs() == 1.0 {
            continue;
        }
        //Temporary fix to skip values for the angle which would produce undefined values. These arise when these specific angles are used in a tan function (line 52)

        let xpos : [f64;SIZE];
        let ypos : [f64;SIZE];
        (xpos,ypos) = calcRayPath(angle,dy);
        // Determines the path of the ray, storing its x positions as 'xpos' and y positions as 'ypos'

        for i in 0..xpos.len(){
            output = output + &xpos[i].to_string() + " " + &(-1.0 * ypos[i]).to_string() + "\n";
        }

        // Adds the positions of the ray to the 'output' variable, which also starts a new line for the positions of the next ray.
    

    
    }
    let real_output = output.as_str();

    let mut file = File::create(format!("dataset{}.txt",1))?;
    file.write_all(real_output.as_bytes())?;
    Ok(())
    // Outputs the positions of the rays into a .txt data file
    
}

