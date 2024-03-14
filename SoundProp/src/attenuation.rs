



// Function to calculate the absorption coefficient A&M (NPL)
fn calculate_absorption(f: f64, temp: f64, salinity: f64, ddepth: f64, ) -> f64 {

    let ph: f64  = 8.0
   
    let z: f64 = 10.0;

    let depth = ddepth / 1000.0;


    let f1 = 0.91 * (salinity / 35.0).sqrt() * (temp/33.0).exp();
    let f2 = 46.0 * (temp/18.0).exp();

    let a = 0.101 * ((f1 * f.powi(2)) / (f1.powi(2) + f.powi(2))) * ((ph - 8.0) / 0.57).exp();
    let b = 0.56 * (1.0 + (temp / 76.0)) * (salinity / 35.0) * ((f2 * f.powi(2)) / (f2.powi(2) + f.powi(2))) * (-depth / 4.9).exp();
    let c = (0.0004937-(2.59 *  z.powf(-5.0)) * temp + 9.11 * z.powf(-7.0) * temp.powi(2) -1.5010 * z.powf(-8.0) * temp.powi(3)) * ((1.0-((3.38 * z.powf(-2.0)) * depth) + (4.9 * z.powf(-4.0) * depth.powi(2))))* f.powi(2);
    //let c = 0.00049 * f.powi(2) * e.powf(-(temp / 27.0 + depth / 17.0));



    (a + b + c) / 1000.0


    
}


fn main() {
    let depth = 0.05;  // metres
    let surface_temp = 20.0;  // degrees C
    let bottom_temp = 4.0;  // degrees C
    let thermocline_start = 200.0;  // metres
    let thermocline_end = 1000.0;  // metres

    let p = 1000.0; // Density in kg/m^3
    let g = 9.81; // Acceleration due to gravity

    let salinity = 35.0; //ppt
    let f = 20.0; //frequency (kHz)
    let ph = 8.0; 

    //for depth in (0..=depth as i32).step_by(10) {
    let pressure = calculate_pressure(depth as f32, p, g); //calculate pressure in Pa
    //println!("Pressure: {} Pa", pressure);  // test output- comment out when in compleded code (DG)


    let temp = temperature_at_depth(depth as f32, surface_temp, bottom_temp, thermocline_start, thermocline_end);
    //println!("The temperature and pressure at {} meters is {} degrees Celsius and {} Pa.", depth, temp, pressure);
    println!("{}", temp);
    let absorp =  calculate_absorption(f, temp, salinity, depth as f32, ph);
    println!("{}",absorp);
   
    
//}
   
   
}
