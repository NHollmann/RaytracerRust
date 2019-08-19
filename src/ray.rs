use crate::vector::Vector3d;

#[derive(Debug)]
pub struct Ray {
    origin : Vector3d,
    direction : Vector3d
}

impl Ray {
    pub fn new(origin : Vector3d, direction : Vector3d) -> Ray {
        Ray {origin, direction}
    }

    pub fn get_origin(&self) -> Vector3d {
        self.origin
    }

    pub fn get_direction(&self) -> Vector3d {
        self.direction
    }

    pub fn point_at(&self, t : f64) -> Vector3d {
        self.origin + self.direction * t
    }
}
