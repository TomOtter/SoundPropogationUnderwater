use std::fs::File;
use std::io::prelude::*;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

fn rIndex(depth: f64) -> f64 {
    let z: f64 = depth;
    let result: f64;
    if z < 0.0 {
        result = 1000.0 / 343.0;
    }
    else if z > 4000.0 {
        result = 1000.0 / 4343.0;
    }
    else{
        result = 1000.0 / (1521.45 - 0.0666*z + 0.0000343*z*z);
    }
    result
}



fn calcRayPath(initialAngle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_directions: [f64;SIZE] = [0.0;SIZE];

    ray_ypositions[0] = 100.0;
    ray_directions[0] = initialAngle;

    let mut stepVector: f64 = dy;

    for i in 0..SIZE-1 {
        ray_xpositions[i+1] = ray_xpositions[i] + stepVector * ray_directions[i].tan();
        ray_ypositions[i+1] = ray_ypositions[i] + stepVector;

        let depth: f64 = ray_ypositions[i];

        if rIndex(depth + stepVector) < rIndex(depth) {
            let criticalAngle : f64 = (rIndex(depth + stepVector)/rIndex(depth)).asin();
            if ray_directions[i].abs() > criticalAngle.abs() {
                ray_directions[i+1] = -1.0 * ray_directions[i];
                stepVector = stepVector * -1.0;
            }
            else {
                let preangle = rIndex(depth)/rIndex(depth+stepVector) * ray_directions[i].sin();
                ray_directions[i+1] = preangle.asin();
            }
        }
        else {
            let preangle = rIndex(depth)/rIndex(depth+stepVector) * ray_directions[i].sin();
            ray_directions[i+1] = preangle.asin();
        }


    }

    (ray_xpositions,ray_ypositions)
}

const SIZE: usize = 8000;

fn main() -> std::io::Result<()> {

    

    
    let dy: f64 = 10.0;

    let mut output : String = "\n".to_string();

    for i in 1..20{
        let angle = ((i) as f64)  * PI/40.0;
        let xpos : [f64;SIZE];
        let ypos : [f64;SIZE];
        (xpos,ypos) = calcRayPath(angle,dy);

        for i in 0..xpos.len(){
            output = output + &xpos[i].to_string() + " " + &ypos[i].to_string() + "\n";
        }

    
    

    }
let real_output = output.as_str();

let mut file = File::create(format!("dataset{}.txt",1))?;
file.write_all(real_output.as_bytes())?;
Ok(())
    
}

