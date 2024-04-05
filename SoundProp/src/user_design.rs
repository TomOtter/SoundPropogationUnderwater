pub const PI: f64 = 3.14159265358979323846264338327950288_f64;

pub struct Simulation {
    pub dt : f64,
    pub water_type : somefunction,
    pub boundary_type: somefunction,
    pub sources : Vec,



}
impl Simulation {
    pub fn calculate(&mut self, duration: f64) {
        for source in self.sources {



        }
        
    }

    pub fn add_source(&mut self) {
        if they wanna add source mid way thru
    }
}






pub struct Source {
    pub mut angle_range : [f64;2] = [-PI,PI],
    pub mut number_of_rays : i32 = 240;
    pub source_type : somefunction,
    pub intensity : f64,

}

impl Source {
    pub fn initialise(&mut self) {
        if self.angle_range[0].abs() > PI {
            println!("USER ERROR - Please give a source angle range between -pi and +pi");
            angle_range = [-PI,PI];
        }

        let self.SIZE : f64 = duration/dt ;
    }


    pub fn calculate(&mut self, duration: f64, dy: f64) -> (Vec,Vec) {
        let mut ray_xpositions: [f64;SIZE] = [0.0;SIZE];
        let mut ray_ypositions: [f64;SIZE] = [0.0;SIZE];


        let mut extraRays : Vec::new();
        let mut extra_ray_times : Vec::new();


        for i in -0..(number_of_rays + 1) {
            let difference : f64 = angle_range[1] - angle_range[0] / (number_of_rays as f64); 
            let initial_angle = angle_range[0] + difference * (i as f64)  ;
            // Sets the initial angle of the ray. The initial angle of each ray increases by pi/120 in each iteration between +/- pi.

            if initial_angle.sin().abs() == 1.0 {
                continue;

            let mut ray1 = ray_trace::Ray {
                angle: initial_angle,
                x_pos: ray_xpositions[0],
                y_pos: ray_ypositions[0],
                intensity: 1.0,
                step_vector: dy,
                frequency: 20.0,
            }; // Defines the initial values of a ray under the ray_trace module
            ray1.initialise(dy);


            for i in 0..SIZE-1 {
                ray1.step();

                // if ray1.extraRayneeded() {
                //     extraRays.push();
                // }


                ray_xpositions[i+1] = ray1.x_pos;
                ray_ypositions[i+1] = ray1.y_pos;
            
            }

            let mut j : usize = 0;
            let mut stop : usize = extraRays.len();
            while j < stop {   

                //make this bit calculate the ray for its remaining time


                j = j + 1;
                stop = extraRays.len()
            }



    }
}
