use renderer::object::{IntersectionResult, Intersectable};
use renderer::ray::Ray;

pub struct Sphere {

}

impl Sphere {

}

impl Intersectable for Sphere {
    fn detect_intersection(&self, ray: Ray) -> Option<IntersectionResult> {
        None
    }
}
