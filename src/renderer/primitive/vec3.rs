use renderer::primitive::point::Point;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn dot(&self, v2: Vec3) -> f32 {
        self.x*v2.x() + self.y*v2.y() + self.z*v2.z()
    }

    pub fn cross(&self, v2: Vec3) -> Vec3 {
        Vec3::new(self.y*v2.z() - self.z*v2.y(),
                  self.z*v2.x() - self.z*v2.z(),
                  self.x*v2.y() - self.y*v2.x())
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        Vec3::new(self.x / length, self.y / length, self.z / length)
    }

    pub fn to_point(self) -> Point {
        Point::new(self.x, self.y, self.z)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x(), self.y + _rhs.y(), self.z + _rhs.z())
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - _rhs.x(), self.y - _rhs.y(), self.z - _rhs.z())
    }
}

impl std::ops::Sub<Point> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Point) -> Vec3 {
        Vec3::new(self.x - _rhs.x(), self.y - _rhs.y(), self.z - _rhs.z())
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x * _rhs.x(), self.y * _rhs.y(), self.z * _rhs.z())
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f32) -> Vec3 {
        Vec3::new(self.x * _rhs, self.y * _rhs, self.z * _rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self * _rhs.x(), self * _rhs.y(), self * _rhs.z())
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f32) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}
