use crate::math::vector::Vector3d;
use crate::math::ray::Ray;
use crate::scene::HitRecord;
use crate::material::Material;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Sphere {
    center : Vector3d,
    radius : f64,
    material: Material,
}

impl Sphere {

    pub fn hit(&self, ray : &Ray, t_min : f64, t_max : f64, record : &mut HitRecord) -> bool {

        let oc = ray.get_origin() - self.center;
        let a = Vector3d::dot(ray.get_direction(), ray.get_direction());
        let b = Vector3d::dot(oc, ray.get_direction());
        let c = Vector3d::dot(oc, oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                record.t = temp;
                record.point = ray.point_at(temp);
                record.normal = (record.point - self.center) / self.radius;
                record.material = self.material;
                return true;
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp > t_min && temp < t_max {
                record.t = temp;
                record.point = ray.point_at(temp);
                record.normal = (record.point - self.center) / self.radius;
                record.material = self.material;
                return true;
            }
        }

        false
    }
}
