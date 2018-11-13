pub mod pinhole_camera;

use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;
use renderer::ray::Ray;

pub trait Camera {
    fn set_location(&mut self, location: Point);
    fn generate_ray(&self, x: f32, y: f32) -> Ray;
}
