use renderer::object::{IntersectionResult, Intersectable, SphericalIntersectable};
use renderer::primitive::point::Point;
use renderer::ray::Ray;

pub struct Sphere {
    origin: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(origin: Point, radius: f32) -> Sphere {
        Sphere {
            origin: origin,
            radius: radius,
        }
    }
}

impl Intersectable for Sphere {
    fn detect_intersection(&self, ray: Ray) -> Option<IntersectionResult> {
        self.detect_spherical_intersection(ray, self.origin, self.radius)
    }
}

impl SphericalIntersectable for Sphere { }
