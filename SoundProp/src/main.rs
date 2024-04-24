use ray_trace::Simulation;
use crate::ray_trace::SourceType::Point;


mod ray_trace;
// Inputs our 'ray_trace' module to this file.

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> std::io::Result<()> {

    let mut sound_prop: Simulation = Simulation::initialise(0.01);
    sound_prop.add_source(-PI, PI, 240, 1.0, 20.0, [-500.0, -100.0], Point);
    sound_prop.add_source(-PI, PI, 240, 1.0, 20.0, [500.0, -100.0], Point);
    sound_prop.calculate(5.0);
    sound_prop.gif();

    Ok(())
    
}