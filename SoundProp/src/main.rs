use std::fs::File;
use std::io::prelude::*;



fn main() -> std::io::Result<()> {

    const SIZE: usize = 200;
    let mut pressuresOld: [f64;SIZE] = [0.0;SIZE]; //sea pressure is 10051.8 Pa
    let mut pressuresCurrent: [f64;SIZE] = [0.0;SIZE];
    let mut pressuresNew: [f64;SIZE] = [0.0;SIZE];

    let speed: f64 = 1500.0; 

    let maxTime = 250;

    let dt: f64 = 0.0001;
    let dx: f64 = 1.0;


    let mut p : f64;


    let mut output : String = "\n".to_string();


    for t in 0 .. (maxTime) {
        for x in 1 .. (SIZE - 1) {  //Can not be end nodes as they have only one neighbor
            pressuresNew[x] = ((dt * dt)/(dx * dx)) * speed * speed * (pressuresCurrent[x + 1] - (2.0 * pressuresCurrent[x]) + (pressuresCurrent[x-1])) - pressuresOld[x] + (2.0 * pressuresCurrent[x]);
        }
        pressuresNew[0] = pressuresNew[1];   //stabilization condition 
        pressuresNew[SIZE - 1] = pressuresNew[SIZE - 2];

       p = -t.exp();
       pressuresNew[SIZE/2] = pressuresNew[SIZE/2] + 10.0 * p;

       if t % 5 == 0 {
           for x in 0 .. (SIZE) {
               output = output + &x.to_string() + " " + &(pressuresNew[x].to_string()) + "\n" ;
           }
           
           output = output + "\n" ;
       }

       pressuresOld = pressuresCurrent;
       pressuresCurrent = pressuresNew;

    }

    let real_output = output.as_str();

    let mut file = File::create("dataset.txt")?;
    file.write_all(real_output.as_bytes())?;
    Ok(())
}
