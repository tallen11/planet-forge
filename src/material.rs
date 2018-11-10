use std::f64::consts;

pub trait Material {
    fn brdf(&self, incoming_angle: f64, outgoing_angle: f64) -> f64;
}

pub struct PhongMaterial {
    diffuse_reflectivity: f64,
    specular_reflectivity: f64,
    specular_exponent: i32,
}

impl PhongMaterial {
    pub fn new(diffuse_reflectivity: f64, specular_reflectivity: f64, specular_exponent: i32) -> PhongMaterial {
        assert!(diffuse_reflectivity + specular_reflectivity <= 1.0);

        PhongMaterial {
            diffuse_reflectivity: diffuse_reflectivity,
            specular_reflectivity: specular_reflectivity,
            specular_exponent: specular_exponent,
        }
    }
}

impl Material for PhongMaterial {
    fn brdf(&self, incoming_angle: f64, outgoing_angle: f64) -> f64 {
        let alpha = 1.0_f64;
        self.diffuse_reflectivity / consts::PI +
                self.specular_reflectivity * ((self.specular_exponent as f64 + 2.0) / (2.0 * consts::PI)) * alpha.cos().powi(self.specular_exponent)
    }
}
