pub mod pinhole_camera;

use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;
use renderer::ray::Ray;

pub trait Camera {
    fn set_origin(&mut self, origin: Point);
    fn generate_ray(&self, u: f32, v: f32) -> Ray;
}
