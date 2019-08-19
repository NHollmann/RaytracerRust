use crate::math::ray::Ray;
use crate::math::vector::Vector3d;

pub struct Camera {
    origin: Vector3d,
    lower_left_corner: Vector3d,
    horizontal: Vector3d,
    vertical: Vector3d,
}

impl Camera {
    pub fn new(look_from : Vector3d, look_at : Vector3d, up : Vector3d, vfov : f64, aspect : f64 ) -> Camera {

        let theta = vfov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = half_height * aspect;

        let w = (look_from - look_at).normalized();
        let u = Vector3d::cross(up, w).normalized();
        let v = Vector3d::cross(w, u);

        let lower_left_corner = look_from - half_width * u - half_height * v - w;

        Camera { 
            origin: look_from,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            lower_left_corner
        }
    }

    pub fn get_ray(&self, u : f64, v : f64) -> Ray {
        let direction = self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin;

        Ray::new(self.origin, direction)
    }
}
