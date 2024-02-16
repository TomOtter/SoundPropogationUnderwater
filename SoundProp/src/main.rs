use std::fs::File;
use std::io::prelude::*;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

fn rIndex(depth: f64) -> f64 {
    let z: f64 = depth;
    let result: f64;
    if z < 0.0 {
        result = 1000.0 / 343.0;
    }
    else if z > 4000.0 {
        result = 1000.0 / 4343.0;
    }
    else{
        result = 1000.0 / (1521.45 - 0.0666*z + 0.0000343*z*z);
    }
    result
}
// Determines the speed of the ray depepndant on the depth 'z'.
// z > 0 -> air,   0 > z > 4000 -> water,   z < 4000 -> sea floor 


fn calcRayPath(initialAngle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_directions: [f64;SIZE] = [0.0;SIZE];

    let mut angle: f64 = initialAngle;
    let mut stepVector: f64 = dy;

    if initialAngle > PI/2.0 {
        stepVector = -1.0 * stepVector;
        angle = (PI - initialAngle)
    }
    else if initialAngle < -PI/2.0 {
        stepVector = -1.0 * stepVector;
        angle = (-PI - initialAngle)
    }
    // Bounds the angle of the ray between +/- pi/2 from the normal and flips the direction of the step vector.

    ray_ypositions[0] = 100.0;
    ray_directions[0] = angle;
    // Sets the starting position and angle of each ray

    

    for i in 0..SIZE-1 {


        ray_xpositions[i+1] = ray_xpositions[i] + stepVector * ray_directions[i].tan();
        ray_ypositions[i+1] = ray_ypositions[i] + stepVector;
        //Calculates the new position of the ray after a step is taken

        let depth: f64 = ray_ypositions[i];

        if rIndex(depth + stepVector) < rIndex(depth) {
            let criticalAngle : f64 = (rIndex(depth + stepVector)/rIndex(depth)).asin();
            if ray_directions[i].abs() > criticalAngle.abs() {
                ray_directions[i+1] = -1.0 * ray_directions[i];
                stepVector = stepVector * -1.0;
            }
            // Reflects the ray if its angle with the normal exceeds the critical angle
                
            else {
                let preangle = rIndex(depth)/rIndex(depth+stepVector) * ray_directions[i].sin();
                ray_directions[i+1] = preangle.asin();
            }
        }
        else {
            let preangle = rIndex(depth)/rIndex(depth+stepVector) * ray_directions[i].sin();
            ray_directions[i+1] = preangle.asin();
        }
        // Refracts the ray if its angle with the normal does not exceed the critical angle

    }

    (ray_xpositions,ray_ypositions)
    // Outputs the x-y positions of the ray for each iterative step
}

const SIZE: usize = 800;

fn main() -> std::io::Result<()> {

    

    
    let dy: f64 = 10.0;
    //Sets step size
    
    let mut output : String = "\n".to_string();

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
            output = output + &xpos[i].to_string() + " " + &ypos[i].to_string() + "\n";
        }
        // Adds the positions of the ray to the 'output' variable, which also starts a new line for the positions of the next ray.
    

    
    

    }
    let real_output = output.as_str();

    let mut file = File::create(format!("dataset{}.txt",1))?;
    file.write_all(real_output.as_bytes())?;
    Ok(())
    // Outputs the positions of the rays into a .txt data file
    
}

