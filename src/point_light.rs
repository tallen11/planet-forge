

use intersectable::{IntersectionResult, Intersectable};
use ray::Ray;

pub struct PointLight {

}

impl PointLight {
    pub fn new() -> PointLight {
        PointLight {

        }
    }
}

impl Intersectable for PointLight {
    fn did_intersect_with_ray(ray: &Ray) -> Option<IntersectionResult> {
        
    }
}
