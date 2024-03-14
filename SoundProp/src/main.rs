use std::fs::File;
use std::io::prelude::*;


mod ray_trace;
// Inputs our 'ray_trace' module to this file.


pub const PI: f64 = 3.14159265358979323846264338327950288_f64;


fn calcRayPath(initial_angle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_directions: [f64;SIZE] = [0.0;SIZE];

    // Defines an array for all of the angles and positions of a single ray

    ray_ypositions[0] = 100.0;

    // Sets initial depth of the point source

    let mut ray1 = ray_trace::Ray {
        angle: initial_angle,
        x_pos: ray_xpositions[0],
        y_pos: ray_ypositions[0],
        intensity: 1.0,
        stepVector: 1.0,
        frequency: 20.0,
    }; // Defines the initial values of a ray under the ray_trace module

    ray1.initialise(dy);
    ray_directions[0] = ray1.angle;


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

