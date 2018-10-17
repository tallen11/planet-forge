use nalgebra::Vector3;

use ray::Ray;
use intersectable::{IntersectionResult, Intersectable};
use material::Material;

pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }
}

impl Intersectable for Sphere {
    fn did_intersect_with_ray(&self, ray: &Ray) -> Option<IntersectionResult> {
        None
    }
}

impl Material for Sphere {
    fn brdf(&self) -> f64 {
        0.0
    }
}
