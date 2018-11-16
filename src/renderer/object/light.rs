use renderer::object::{IntersectionResult, Intersectable, ObjectType};
use renderer::primitive::point::Point;
use renderer::ray::Ray;
use renderer::material::material::Material;

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
        let oc = ray.get_origin() - self.origin;

        let a = ray.get_direction().dot(ray.get_direction());
        let b = oc.dot(ray.get_direction());
        let c = oc.dot(oc) - self.radius*self.radius;

        let discriminant = b*b - a*c;
        // println!("{}", discriminant);
        if discriminant > 0.0 {
            let t1 = (-b - discriminant.sqrt()) / a;
            let t2 = (-b + discriminant.sqrt()) / a;
            if t1 >= 0.0 && t1 < t2 {
                let ip = ray.p(t1);
                let norm = (ip - self.origin).normalized();
                return Some(IntersectionResult::new(t1, ip.to_point(), norm, self));
            } else if t2 >= 0.0 {
                let ip = ray.p(t2);
                let norm = (ip - self.origin).normalized();
                return Some(IntersectionResult::new(t2, ip.to_point(), norm, self));
            }
        }

        None
    }

    fn get_type(&self) -> ObjectType {
        ObjectType::Light(self.emission)
    }
}
