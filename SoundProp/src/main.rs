use user_design::Simulation;
use crate::user_design::SourceType::Point;


mod ray_trace;
mod user_design;
// Inputs our 'ray_trace' module to this file.

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> std::io::Result<()> {

    let mut sound_prop: Simulation = Simulation::initialise(0.0001);
    sound_prop.add_source(-PI, PI, 240, 1.0, 20.0, [0.0, -10.0], Point);
    sound_prop.calculate(0.1);
    sound_prop.gif();

    Ok(())
    
}