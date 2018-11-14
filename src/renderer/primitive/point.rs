use renderer::primitive::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn zero() -> Point {
        Point {
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

    pub fn to_vec3(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, _rhs: Point) -> Point {
        Point::new(self.x + _rhs.x(), self.y + _rhs.y(), self.z + _rhs.z())
    }
}

impl std::ops::Add<Vec3> for Point {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + _rhs.x(), self.y + _rhs.y(), self.z + _rhs.z())
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vec3;

    fn sub(self, _rhs: Point) -> Vec3 {
        Vec3::new(self.x - _rhs.x(), self.y - _rhs.y(), self.z - _rhs.z())
    }
}
