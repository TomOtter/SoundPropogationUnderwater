//pressure depth dependence with constant water density
fn calculate_pressure(depth: f32, p: f32, g: f32) -> f32 {
    depth * p * g 
}

fn temperature_at_depth(depth: f32, surface_temp: f32, bottom_temp: f32, thermocline_start: f32, thermocline_end: f32) -> f32 {

  //constant temperature up to 200m based on literature - thermocline start depth
    if depth <= thermocline_start {
        surface_temp
    } else if depth >= thermocline_end {
        bottom_temp
    } else {
        // Linear interpolation within the thermocline
        let fraction = (depth - thermocline_start) / (thermocline_end - thermocline_start);
        surface_temp + fraction * (bottom_temp - surface_temp)
    }

}

fn main() {
    let depth = 220.0;  // metres
    let surface_temp = 20.0;  // degrees C
    let bottom_temp = 4.0;  // degrees C
    let thermocline_start = 200.0;  // metres
    let thermocline_end = 1000.0;  // metres

    let p = 1000.0; // Density in kg/m^3
    let g = 9.81; // Acceleration due to gravity

    //for loop depth must be of type i32
    for depth in (0..=depth as i32).step_by(10) {
    let pressure = calculate_pressure(depth as f32, p, g); //calculate pressure in Pa
    

    let temp = temperature_at_depth(depth as f32, surface_temp, bottom_temp, thermocline_start, thermocline_end);

    //Test the correct output - remove when combined with main code    
    println!("The temperature and pressure at {} metres is {} degrees Celsius and {} Pa.", depth, temp, pressure);
    }
}