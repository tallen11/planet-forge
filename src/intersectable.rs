use nalgebra::Vector3;

use ray::Ray;

#[derive(Copy, Clone, Debug)]
pub enum IntersectableType {
    Solid,
    Light(f64),
}

#[derive(Copy, Clone, Debug)]
pub struct IntersectionResult {
    pub i_type: IntersectableType,
    pub t: f64,
    pub intersection_point: Vector3<f64>,
    pub normal: Vector3<f64>,
}

impl IntersectionResult {
    pub fn new(i_type: IntersectableType, t: f64, intersection_point: Vector3<f64>, normal: Vector3<f64>) -> IntersectionResult {
        IntersectionResult {
            i_type: i_type,
            t: t,
            intersection_point: intersection_point,
            normal: normal,
        }
    }
}

pub trait Intersectable {
    fn did_intersect_with_ray(&self, ray: &Ray) -> Option<IntersectionResult>;
}
