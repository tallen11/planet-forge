use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction.normalize(),
        }
    }

    pub fn get_origin(&self) -> Vector3<f64> {
        self.origin
    }

    pub fn get_direction(&self) -> Vector3<f64> {
        self.direction
    }

    pub fn p(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}
