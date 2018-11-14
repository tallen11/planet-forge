pub mod sphere;
pub mod light;

use renderer::primitive::point::Point;
use renderer::primitive::vec3::Vec3;
use renderer::ray::Ray;

#[derive(Copy, Clone, Debug)]
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

pub trait SphericalIntersectable {
    fn detect_spherical_intersection(&self, ray: Ray, origin: Point, radius: f32) -> Option<IntersectionResult> {
        let oc = ray.get_origin() - origin;

        let a = ray.get_direction().dot(ray.get_direction());
        let b = oc.dot(ray.get_direction());
        let c = oc.dot(oc) - radius*radius;

        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t2 {
                let ip = ray.p(t1);
                let norm = (ip - origin).normalized();
                return Some(IntersectionResult::new(t1, ip.to_point(), norm));
            } else {
                let ip = ray.p(t2);
                let norm = (ip - origin).normalized();
                return Some(IntersectionResult::new(t1, ip.to_point(), norm));
            }
        }

        None
    }
}
