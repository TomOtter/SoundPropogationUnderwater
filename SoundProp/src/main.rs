use std::fs::File;
use std::io::prelude::*;


mod ray_trace;
mod user_design;
// Inputs our 'ray_trace' module to this file.

// use crate::ray_trace::material_speed;


pub const PI: f64 = 3.14159265358979323846264338327950288_f64;




let source1 = user_design::Source {

}








fn calc_ray_path(initial_angle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];

    // Defines an array for all of the angles and positions of a single ray

    ray_ypositions[0] = 100.0;

    // Sets initial depth of the point source

    let mut ray1 = ray_trace::Ray {
        angle: initial_angle,
        x_pos: ray_xpositions[0],
        y_pos: ray_ypositions[0],
        intensity: 1.0,
        step_vector: dy,
        frequency: 20.0,
    }; // Defines the initial values of a ray under the ray_trace module

    ray1.initialise(dy);


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
    

    let dy: f64 = 0.01;
    
    //Sets step size

    let mut xoutputs : [[f64;SIZE];241] = [[0.0;SIZE];241];
    let mut youtputs : [[f64;SIZE];241] = [[0.0;SIZE];241];
    
    
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
        (xpos,ypos) = calc_ray_path(angle,dy);
        // Determines the path of the ray, storing its x positions as 'xpos' and y positions as 'ypos'


        let index: usize = (i + 120 as i32) as usize;

        xoutputs[index] = xpos;
        youtputs[index] = ypos; 
    

        // Adds the positions of the ray to the 'output' variable, which also starts a new line for the positions of the next ray.
    }


    let mut output : String = "\n".to_string();


    
    for j in 0..xoutputs[0].len() {
        for i in 0..241 {

            let xpos : f64 = xoutputs[i][j];
            let ypos : f64 = youtputs[i][j];
            output = output + &xpos.to_string() + " " + &(-1.0 * ypos).to_string() + "\n";

        }
        let real_output = output.as_str();
        let mut file = File::create(format!("dataset{}.txt",j))?;
        file.write_all(real_output.as_bytes())?;
        output = "\n".to_string();
        
    }

    Ok(())
    
}



// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
