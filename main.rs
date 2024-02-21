use std::fs::File;
use std::io::prelude::*;

pub const PI: f64 = 3.14159265358979323846264338327950288_f64; // 3.1415926535897931f64

fn velocityWater(_z: f64) -> f64 {
    let _t: f64 = 8.8; //temperature from lit
    let _s: f64 = 22.0; //salinity from lit
    let _l: f64 = 43.0; //latitude from lit
    let _v: f64;
    _v = 1402.5+5.0*_t-5.44e-2*_t*_t+2.1e-4*_t*_t+1.33*_s-1.23e-2*_s*_t+8.7e-5*_s*_t*_t +1.56e-2*_z+2.55e-7*_z*_z-7.3e-12*_z*_z*_z+1.2e-6*_z*(_l-45.0)-9.5e-13*_t*_z*_z*_z+3e-7*_t*_t*_z+1.43e-5*_s*_z;
    _v
}

fn velocitySilt(_z: f64) -> f64 {
  //to define BCs //need equation for velocity in silt
  7.0 //random number for compile lol!
}

fn boundaryConditions(x:f64, y:f64) -> f64{ //doing in 2D //think about adding xboundary and yboundary into main func
    
    let mut xInsideBoundary: bool;
    let mut yInsideBoundary: bool;
    let mut xBoundary: f64 = 1000.0;
    let mut yBoundary: f64 = 1000.0;
    let mut v: f64; //local velocity

    if x < xBoundary {
        xInsideBoundary= true;
    }
    else {
        xInsideBoundary= false;
    }

    if y < yBoundary {
        yInsideBoundary= true;
    }
    else {
        yInsideBoundary= false;
    }
    if xInsideBoundary&yInsideBoundary == true { // & means and.
        velocityWater(y)
    }
    else {
        velocitySilt(y)
    }

    // note: this is not finished yet. 
}

fn rIndex(depth: f64) -> f64 {
    let y: f64 = depth;
    let result: f64;
    if y < 0.0 { //depth is in m
        result = 1000.0 / 343.0;//refrac index
    }
    else if y > 4000.0 {
        result = 1000.0 / 4343.0;//refrac index
    }
    else{
        result = velocityWater(y)  //speed of sound in water //check this func works
    }
    result
}

fn calcRayPath(initialAngle :f64, dy: f64) -> ([f64;SIZE],[f64;SIZE]) {

    let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];
    let mut ray_directions: [f64;SIZE] = [0.0;SIZE];

    let mut angle: f64 = initialAngle;
    let mut stepVector: f64 = dy;

    if initialAngle > PI/2.0 {
        stepVector = -1.0 * stepVector;
        angle = (PI - initialAngle)
    }
    else if initialAngle < -PI/2.0 {
        stepVector = -1.0 * stepVector;
        angle = (-PI - initialAngle)
    }


    ray_ypositions[0] = 100.0;
    ray_directions[0] = angle;


    

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

const SIZE: usize = 800;


fn main() -> std::io::Result<()> {

    

    
    let dy: f64 = 10.0;

    let mut output : String = "\n".to_string();

    for i in -120..121{
        let angle = ((i) as f64)  * PI/120.00 ;
        if angle.sin().abs() == 1.0 {
            continue;
        }

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

