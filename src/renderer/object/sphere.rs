use renderer::object::{IntersectionResult, Intersectable};
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
        let oc = ray.get_origin() - self.origin;

        let a = ray.get_direction().dot(ray.get_direction());
        let b = oc.dot(ray.get_direction());
        let c = oc.dot(oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 < t2 {
                let ip = ray.p(t1);
                let norm = (ip - self.origin).normalized();
                return Some(IntersectionResult::new(t1, ip.to_point(), norm));
            } else {
                let ip = ray.p(t2);
                let norm = (ip - self.origin).normalized();
                return Some(IntersectionResult::new(t1, ip.to_point(), norm));
            }
        }

        None
    }
}
