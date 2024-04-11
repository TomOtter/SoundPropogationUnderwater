use user_design::Simulation;
use crate::user_design::Source_Type::point;


mod ray_trace;
mod user_design;
// Inputs our 'ray_trace' module to this file.

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> std::io::Result<()> {

    let mut SoundProp: Simulation = Simulation::initialise(0.0001);
    SoundProp.add_source(-PI, PI, 240, 1.0, 20.0, [0.0, -10.0], point);
    SoundProp.calculate(0.1);

    Ok(())
    
}



// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }




// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

