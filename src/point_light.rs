use nalgebra::Vector3;

use intersectable::{IntersectableType, IntersectionResult, Intersectable};
use ray::Ray;

pub struct PointLight {
    origin: Vector3<f64>,
    radius: f64,
}

impl PointLight {
    pub fn new(origin: Vector3<f64>, radius: f64) -> PointLight {
        PointLight {
            origin: origin,
            radius: radius,
        }
    }
}

impl Intersectable for PointLight {
    fn did_intersect_with_ray(&self, ray: &Ray) -> Option<IntersectionResult> {
        let omc = ray.get_origin() - self.origin;

        let a = 1.0;
        let b = 2.0 * ray.get_direction().dot(&omc);
        let c = omc.dot(&omc) - self.radius*self.radius;

        let root_check = b*b - 4.0*a*c;
        if root_check <= 0.0 {
            return None;
        }

        let t_0 = (-b + root_check.sqrt()) / (2.0 * a);
        let t_1 = (-b - root_check.sqrt()) / (2.0 * a);
        let t = if t_0 < t_1 { t_1 } else { t_0 };
        let intersection_point = ray.p(t);
        let normal = (intersection_point - self.origin) / self.radius;

        Some(IntersectionResult::new(IntersectableType::Light(1.0), t, intersection_point, normal))
    }
}
