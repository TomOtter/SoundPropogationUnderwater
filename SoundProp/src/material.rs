#[derive(Clone, Copy, Debug)]
pub enum MaterialType {
    // Stone
    Basalt,
    Granite,
    Quartzite,
    Gneiss,
    Schist,
    Marble,
    Limestone,
    Shale,
    Sandstone,
    // Other Material
    TurbiditeArea,
    SiliceousSediment,
    CalcerousSediment,
    Sand,
}

#[derive(Clone, Copy)]
pub struct Material {
    material_name: MaterialType,
    shear_modulus: Option<f64>,
    bulk_modulus: Option<f64>,
    density: Option<f64>,
}

impl Material {
    pub fn define(material: MaterialType) -> Self {
        match material {
            MaterialType::Basalt => {
                Self {
                    material_name: material,
                    shear_modulus: Some(62.6*1000000000.0/2.0*(1.0+0.25)),
                    bulk_modulus: Some(62.6*1000000000.0/3.0*(1.0+2.0*0.25)),
                    density: Some(3011.0),  
                }
             }
             MaterialType::Granite => {
                Self {
                    material_name: material,
                    shear_modulus: Some(59.3*1000000000.0/2.0*(1.0+0.23)),
                    bulk_modulus: Some(59.3*1000000000.0/3.0*(1.0+2.0*0.23)),
                    density: Some(2691.0), 
                }
             }
             MaterialType::Quartzite => {
                Self {
                    material_name: material,
                    shear_modulus: Some(70.9*1000000000.0/2.0*(1.0+0.15)),
                    bulk_modulus: Some(70.9*1000000000.0/3.0*(1.0+2.0*0.15)),
                    density: Some(2655.0),  
                }
             }
             MaterialType::Gneiss=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(58.6*1000000000.0/2.0*(1.0+0.21)),
                    bulk_modulus: Some(58.6*1000000000.0/3.0*(1.0+2.0*0.21)),
                    density: Some(2750.0), 
                }
            }
            MaterialType::Schist=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(42.4*1000000000.0/2.0*(1.0+0.12)),
                    bulk_modulus: Some(42.4*1000000000.0/3.0*(1.0+2.0*0.12)),
                    density: Some(2350.0), 
                }
            }
            MaterialType::Marble=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(46.3*1000000000.0/2.0*(1.0+0.23)),
                    bulk_modulus: Some(46.3*1000000000.0/3.0*(1.0+2.0*0.23)),
                    density: Some(2711.0), 
                }
            }
            MaterialType::Limestone=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(50.4*1000000000.0/2.0*(1.0+0.25)),
                    bulk_modulus: Some(50.4*1000000000.0/3.0*(1.0+2.0*0.25)),
                    density: Some(1790.0), 
                }
            }
            MaterialType::Shale=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(13.7*1000000000.0/2.0*(1.0+0.08)),
                    bulk_modulus: Some(13.7*1000000000.0/3.0*(1.0+2.0*0.08)),
                    density: Some(2675.0), 
                }
            }
            MaterialType::Sandstone=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(15.3*1000000000.0/2.0*(1.0+0.24)),
                    bulk_modulus: Some(15.3*1000000000.0/3.0*(1.0+2.0*0.24)),
                    density: Some(2323.0), 
                }
            }
            MaterialType::TurbiditeArea |
            MaterialType::SiliceousSediment |
            MaterialType::CalcerousSediment |
            MaterialType::Sand => {
                Self{
                    material_name: material,
                    shear_modulus: None,
                    bulk_modulus: None,
                    density: None, 
                }
            }
        }
    }

    pub fn calculate_velocity(&self, depth: f64) -> f64 {
        match self.material_name {
            // Stone Materials
            MaterialType::Basalt |
            MaterialType::Granite |
            MaterialType::Quartzite |
            MaterialType::Gneiss |
            MaterialType::Schist |
            MaterialType::Marble |
            MaterialType::Limestone |
            MaterialType::Shale |
            MaterialType::Sandstone => ((self.bulk_modulus.unwrap() + (1.333333333333 * self.shear_modulus.unwrap())) / self.density.unwrap()).sqrt(),
            MaterialType::TurbiditeArea => (1.511+ 1.304*depth*0.001 - 0.257*depth*depth*depth*0.001*0.001*0.001)*1000.0,
            MaterialType::SiliceousSediment => (1.509 + 0.869*depth*0.001 - 0.267*depth*depth*0.001*0.001)*1000.0,
            MaterialType::CalcerousSediment => (1.559 + 1.713*depth*0.001 - 0.374*depth*depth*0.001*0.001)*1000.0,
            MaterialType::Sand => 1626.0,
        }
    }
}