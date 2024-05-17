


fn main() {
    StoneSoundVelocity(defineMaterial(Material::Granite(())))
}
pub enum Material {
    Basalt(f64),
    Granite(f64),
    Quartzite(f64),
    Gneiss(f64),
    Schist(f64),
    Marble(f64),
    Limestone(f64),
    Shale(f64),
    Sandstone(f64),

}

struct StoneSoundVelocity {
    shearModulus: f64,
    bulkModulus: f64,
    density: f64,
}

impl StoneSoundVelocity {
    pub fn defineMaterial(material: Material) -> Self {
        match material {
            Basalt => {
                Self {
                    shearModulus: 62.6*1000000000.0/2.0*(1.0+0.25),
                    bulkModulus: 62.6*1000000000.0/3.0*(1.0+2.0*0.25),
                    density: 3011.0,  
                }
             }
            Granite => {
                Self {
                    shearModulus: 59.3*1000000000.0/2.0*(1.0+0.23),
                    bulkModulus: 59.3*1000000000.0/3.0*(1.0+2.0*0.23),
                    density: 2691.0,  
                }
             }
            Quartzite => {
                Self {
                    shearModulus: 70.9*1000000000.0/2.0*(1.0+0.15),
                    bulkModulus: 70.9*1000000000.0/3.0*(1.0+2.0*0.15),
                    density: 2655.0,  
                }
             }
            Gneiss=> {
                Self{
                    shearModulus: 58.6*1000000000.0/2.0*(1.0+0.21),
                    bulkModulus: 58.6*1000000000.0/3.0*(1.0+2.0*0.21),
                    density: 2750.0, 
                }
            }
            Schist=> {
                Self{
                    shearModulus: 42.4*1000000000.0/2.0*(1.0+0.12),
                    bulkModulus: 42.4*1000000000.0/3.0*(1.0+2.0*0.12),
                    density: 2350.0, 
                }
            }
            Marble=> {
                Self{
                    shearModulus: 46.3*1000000000.0/2.0*(1.0+0.23),
                    bulkModulus: 46.3*1000000000.0/3.0*(1.0+2.0*0.23),
                    density: 2711.0, 
                }
            }
            Limestone=> {
                Self{
                    shearModulus: 50.4*1000000000.0/2.0*(1.0+0.25),
                    bulkModulus: 50.4*1000000000.0/3.0*(1.0+2.0*0.25),
                    density: 1790.0, 
                }
            }
            Shale=> {
                Self{
                    shearModulus: 13.7*1000000000.0/2.0*(1.0+0.08),
                    bulkModulus: 13.7*1000000000.0/3.0*(1.0+2.0*0.08),
                    density: 2675.0, 
                }
            }
            Sandstone=> {
                Self{
                    shearModulus: 15.3*1000000000.0/2.0*(1.0+0.24),
                    bulkModulus: 15.3*1000000000.0/3.0*(1.0+2.0*0.24),
                    density: 2323.0, 
                }
            }
        }
    }
      pub fn velocityCalculation(material: MaterialType){
        let velocity = ((self.bulkModulus + (1.333333333333*self.shearModulus))/self.density).sqrt();
}




