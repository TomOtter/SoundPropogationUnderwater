use ray_trace::Simulation;
use crate::ray_trace::SourceType::Point;


mod ray_trace;
// Inputs our 'ray_trace' module to this file.

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> std::io::Result<()> {

    let mut sound_prop: Simulation = Simulation::initialise(0.01, 0.1, [-1500.0,1500.0], [-2000.0,1000.0]);
    sound_prop.add_source(-PI, PI, 240, 1.0, 20.0, [-500.0, -100.0], Point);
    sound_prop.add_source(-PI, PI, 240, 1.0, 20.0, [500.0, -100.0], Point);
    sound_prop.gif(2.0, 100);

    Ok(())
    
}