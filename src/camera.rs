use nalgebra::Vector3;

use ray::Ray;

pub trait Camera {
    fn generate_ray(&self, x: f64, y: f64) -> Ray;
}

pub struct BasicCamera {
    origin: Vector3<f64>,
    near_plane_distance: f64,
}

impl BasicCamera {
    pub fn new() -> BasicCamera {
        BasicCamera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            near_plane_distance: 1.0,
        }
    }
}

impl Camera for BasicCamera {
    fn generate_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(self.origin, Vector3::new(y, x, self.near_plane_distance))
    }
}
