use crate::math::vector3d::Vector3d;

#[derive(Debug)]
pub struct Ray {
    origin : Vector3d,
    direction : Vector3d
}

impl Ray {
    pub fn new(origin : Vector3d, direction : Vector3d) -> Ray {
        Ray {origin, direction}
    }
}
