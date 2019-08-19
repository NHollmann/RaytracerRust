
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Vector3d {
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

impl Vector3d {
    pub fn new(x : f64, y : f64, z : f64) -> Vector3d {
        Vector3d { x, y, z }
    }

    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f64 {
         self.len_sqr().sqrt()
    }

    pub fn normalized(&self) -> Vector3d {
        let len = self.len();

        Vector3d {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(left : Vector3d, right : Vector3d) -> f64 {
        left.x * right.x + left.y * right.y + left.z * right.z
    }

    pub fn cross(left : Vector3d, right : Vector3d) -> Vector3d {
        Vector3d {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}

impl std::ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::Mul<Vector3d> for f64 {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl std::ops::Mul<f64> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f64) -> Vector3d {
        Vector3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
