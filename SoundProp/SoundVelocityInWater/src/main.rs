fn main() {
    let y=  speed(8.8, 22.0, 1000.0, 43.0);
    println!("Speed of sound is: {y}")
    //Speed in m/s
    //put depth function in here
}

fn speed(_t: f64, _s: f64, _z: f64, _l: f64) -> f64 {
 1402.5+5.0*_t-5.44e-2*_t*_t+2.1e-4*_t*_t+1.33*_s-1.23e-2*_s*_t+8.7e-5*_s*_t*_t+1.56e-2*_z+2.55e-7*_z*_z-7.3e-12*_z*_z*_z+1.2e-6*_z*(_l-45.0)-9.5e-13*_t*_z*_z*_z+3e-7*_t*_t*_z+1.43e-5*_s*_z
   //put depth function in here
}

fn temperature(_z: f64) {
    //put in depth function here
}

fn salinity(_z: f64) {
    //put in depth function here 
}

fn depth(_z: f64){

}