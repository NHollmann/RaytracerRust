use crate::math::vector::Vector3d;
use crate::math::ray::Ray;

pub struct Sphere {
    center : Vector3d,
    radius : f64
}

impl Sphere {
    pub fn new(center : Vector3d, radius : f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }

    pub fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> bool {

        let oc = ray.get_origin() - self.center;
        let a = Vector3d::dot(ray.get_direction(), ray.get_direction());
        let b = Vector3d::dot(oc, ray.get_direction());
        let c = Vector3d::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return true;
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                return true;
            }
        }

        false
    }
}
