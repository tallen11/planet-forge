use renderer::camera::Camera;
use renderer::ray::Ray;
use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;

#[derive(Copy, Clone)]
pub struct PinholeCamera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Point,
}

impl PinholeCamera {
    pub fn new() -> PinholeCamera {
        let focal_length = 2.0 / (std::f32::consts::PI / 2.0).atan();
        PinholeCamera {
            lower_left_corner: Vec3::new(-2.0, -1.0, focal_length),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Point::zero(),
        }
    }
}

impl Camera for PinholeCamera {
    fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    fn generate_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
