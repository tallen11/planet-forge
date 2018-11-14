use renderer::object::{IntersectionResult, Intersectable, SphericalIntersectable};
use renderer::primitive::point::Point;
use renderer::ray::Ray;

pub struct Light {
    origin: Point,
    radius: f32,
    emission: f32,
}

impl Light {
    pub fn new(origin: Point, radius: f32) -> Light {
        Light {
            origin: origin,
            radius: radius,
            emission: 1.0,
        }
    }
}

impl Intersectable for Light {
    fn detect_intersection(&self, ray: Ray) -> Option<IntersectionResult> {
        self.detect_spherical_intersection(ray, self.origin, self.radius)
    }
}

impl SphericalIntersectable for Light { }
