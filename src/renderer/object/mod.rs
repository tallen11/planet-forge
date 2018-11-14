pub mod sphere;
pub mod light;

use renderer::primitive::point::Point;
use renderer::primitive::vec3::Vec3;
use renderer::ray::Ray;

pub struct IntersectionResult {
    t: f32,
    intersection_point: Point,
    intersection_normal: Vec3,
}

impl IntersectionResult {
    pub fn new(t: f32, intersection_point: Point, intersection_normal: Vec3) -> IntersectionResult {
        IntersectionResult {
            t: t,
            intersection_point: intersection_point,
            intersection_normal: intersection_normal,
        }
    }

    pub fn get_t(&self) -> f32 {
        self.t
    }

    pub fn get_intersection_point(&self) -> Point {
        self.intersection_point
    }

    pub fn get_intersection_normal(&self) -> Vec3 {
        self.intersection_normal
    }
}

pub trait Intersectable {
    fn detect_intersection(&self, ray: Ray) -> Option<IntersectionResult>;
}
