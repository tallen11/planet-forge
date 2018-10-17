use nalgebra::Vector3;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction,
        }
    }

    pub fn p(&self, t: f64) -> Vector3<f64> {
        self.origin + t * self.direction
    }
}
