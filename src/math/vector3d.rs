
#[derive(Debug)]
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
}

impl std::ops::Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
