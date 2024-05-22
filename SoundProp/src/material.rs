#[derive(Clone, Copy, Debug, PartialEq)]
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
    // Sediment Material
    TurbiditeArea,
    SiliceousSediment,
    CalcerousSediment,
    Sand,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    material_name: MaterialType,
    shear_modulus: Option<f64>,
    bulk_modulus: Option<f64>,
    density: Density,
}

impl Material {
    pub fn define(material: MaterialType) -> Self {
        match material {
            MaterialType::Basalt => {
                Self {
                    material_name: material,
                    shear_modulus: Some(62.6*1000000000.0/2.0*(1.0+0.25)),
                    bulk_modulus: Some(62.6*1000000000.0/3.0*(1.0+2.0*0.25)),
                    density: Density::Constant(3011.0),  
                }
             }
             MaterialType::Granite => {
                Self {
                    material_name: material,
                    shear_modulus: Some(59.3*1000000000.0/2.0*(1.0+0.23)),
                    bulk_modulus: Some(59.3*1000000000.0/3.0*(1.0+2.0*0.23)),
                    density: Density::Constant(2691.0), 
                }
             }
             MaterialType::Quartzite => {
                Self {
                    material_name: material,
                    shear_modulus: Some(70.9*1000000000.0/2.0*(1.0+0.15)),
                    bulk_modulus: Some(70.9*1000000000.0/3.0*(1.0+2.0*0.15)),
                    density: Density::Constant(2655.0),  
                }
             }
             MaterialType::Gneiss=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(58.6*1000000000.0/2.0*(1.0+0.21)),
                    bulk_modulus: Some(58.6*1000000000.0/3.0*(1.0+2.0*0.21)),
                    density: Density::Constant(2750.0), 
                }
            }
            MaterialType::Schist=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(42.4*1000000000.0/2.0*(1.0+0.12)),
                    bulk_modulus: Some(42.4*1000000000.0/3.0*(1.0+2.0*0.12)),
                    density: Density::Constant(2350.0), 
                }
            }
            MaterialType::Marble=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(46.3*1000000000.0/2.0*(1.0+0.23)),
                    bulk_modulus: Some(46.3*1000000000.0/3.0*(1.0+2.0*0.23)),
                    density: Density::Constant(2711.0), 
                }
            }
            MaterialType::Limestone=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(50.4*1000000000.0/2.0*(1.0+0.25)),
                    bulk_modulus: Some(50.4*1000000000.0/3.0*(1.0+2.0*0.25)),
                    density: Density::Constant(1790.0), 
                }
            }
            MaterialType::Shale=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(13.7*1000000000.0/2.0*(1.0+0.08)),
                    bulk_modulus: Some(13.7*1000000000.0/3.0*(1.0+2.0*0.08)),
                    density: Density::Constant(2675.0), 
                }
            }
            MaterialType::Sandstone=> {
                Self{
                    material_name: material,
                    shear_modulus: Some(15.3*1000000000.0/2.0*(1.0+0.24)),
                    bulk_modulus: Some(15.3*1000000000.0/3.0*(1.0+2.0*0.24)),
                    density: Density::Constant(2323.0), 
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
                    density: Density::Variable, 
                }
            }
        }
    }

    pub fn calculate_velocity(&mut self, depth: f64) -> f64 {
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
            MaterialType::Sandstone => ((self.bulk_modulus.unwrap() + (1.333333333333 * self.shear_modulus.unwrap())) / self.calculate_density(0.0, 0.0)).sqrt(),
            // Sediment Materials
            MaterialType::TurbiditeArea => (1.511+ 1.304*depth*0.001 - 0.257*(depth*0.001).powi(3))*1000.0,
            MaterialType::SiliceousSediment => (1.509 + 0.869*depth*0.001 - 0.267*(depth*0.001).powi(2))*1000.0,
            MaterialType::CalcerousSediment => (1.559 + 1.713*depth*0.001 - 0.374*(depth*0.001).powi(2))*1000.0,
            MaterialType::Sand => 1626.0,
        }
    }

    pub fn acoustic_impedance(&mut self, speed_of_sound: f64, depth: f64, boundary_height: f64) -> f64 {
        self.calculate_density(depth, boundary_height) * speed_of_sound
    }

    fn calculate_density(&mut self, depth: f64, boundary_height: f64) -> f64 {
        match self.density {
            Density::Constant(value) => value,
            Density::Variable => {
               let boundary_depth = depth + boundary_height;
               if boundary_depth < 0.0 { 1.66 - depth * 0.000051 + 0.0037 * boundary_depth.abs().powf(0.766) }
               else { 1.66 - depth * 0.000051 + 0.0037 * -boundary_depth.powf(0.766) }
            }
        }
        // Issue with this, boundary_depth part
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Density {
    Constant(f64),
    Variable,
}