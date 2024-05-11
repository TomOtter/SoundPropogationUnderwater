use ray_trace::{
    Simulation,
    SourceType::Point,
};


mod ray_trace;
// Inputs our 'ray_trace' module to this file.

pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

fn main() -> std::io::Result<()> {

    let mut sound_prop = Simulation::initialise(0.01, 5.0, [-1500.0,1500.0], [-2000.0,1000.0]);
    sound_prop.add_boundary(|x: f64| (x/30.0).powi(2) - 1500.0);
    // sound_prop.y_upper_limit(500.0);
    sound_prop.add_source(-PI, PI, 1000, 2.0,
        20.0, [-500.0, -200.0], Point);
   sound_prop.add_source(-PI, PI, 1000, 2.0,
        10.0, [500.0, -200.0], Point);
   sound_prop.gif(2.0, 0.005, 100);

    Ok(())
    
}