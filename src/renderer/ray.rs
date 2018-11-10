use renderer::primitive::point::Point;
use renderer::primitive::vec3::Vec3;

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        Ray {
            origin: origin,
            direction: direction.normalized(),
        }
    }

    pub fn get_origin(&self) -> Point {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn p(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
