use std::fs::File;
use std::io::prelude::*;

use user_design::Simulation;
use crate::user_design::Source_Type::point;


mod ray_trace;
mod user_design;
// Inputs our 'ray_trace' module to this file.

// use crate::ray_trace::material_speed;


pub const PI: f64 = 3.14159265358979323846264338327950288_f64;


const SIZE: usize = 200;

fn main() -> std::io::Result<()> {

    let mut xoutputs: Vec<Vec<f64>> = Vec::new();
    let mut youtputs: Vec<Vec<f64>> = Vec::new();

    let mut SoundProp: Simulation = user_design::Simulation{
        dt: 0.0001,
        sources: Vec::new(),
    };

    SoundProp.add_source(-PI, PI, 240, 1.0, 20.0, [0.0, 10.0], point);
    (xoutputs, youtputs) = SoundProp.calculate(0.1);


    let mut output : String = "\n".to_string();

    for i in 0..xoutputs.len()-1 {
        for j in 0..xoutputs[i].len()-1 {

            let xpos : f64 = xoutputs[i][j];
            let ypos : f64 = youtputs[i][j];
            output = output + &xpos.to_string() + " " + &(-1.0 * ypos).to_string() + "\n";

        }
        let real_output = output.as_str();
        let mut file = File::create(format!("dataset{}.txt",i))?;
        file.write_all(real_output.as_bytes())?;
        output = "\n".to_string();
        
    }

    Ok(())
    
}



// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

