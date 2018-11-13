use renderer::camera::Camera;
use renderer::ray::Ray;
use renderer::primitive::vec3::Vec3;
use renderer::primitive::point::Point;

pub struct PinholeCamera {
    location: Point,
    focal_length: f32,
}

impl PinholeCamera {
    pub fn new() -> PinholeCamera {
        PinholeCamera {
            location: Point::zero(),
            focal_length: 1.0,
        }
    }
}

impl Camera for PinholeCamera {
    fn set_location(&mut self, location: Point) {
        self.location = location;
    }

    fn generate_ray(&self, x: f32, y: f32) -> Ray {
        Ray::new(self.location, Vec3::new(x, y, self.location.z() + self.focal_length))
    }
}
