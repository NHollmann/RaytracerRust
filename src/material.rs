use crate::vector::Vector3d;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::scene::HitRecord;
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Material {
    ambient_color : Vector3d,
    diffuse_color : Vector3d,
    specular_color : Vector3d,
    shininess : f64,

    pub reflection_color : Vector3d,
    pub transmission_color : Vector3d,
}

impl Material {
    pub fn empty() -> Material {
        Material {
            ambient_color: Vector3d::zero(),
            diffuse_color: Vector3d::zero(),
            specular_color: Vector3d::zero(),
            shininess: 0.0,

            reflection_color: Vector3d::zero(),
            transmission_color: Vector3d::zero(),
        }
    }

    pub fn has_brdf(&self) -> bool {
        !(self.ambient_color.is_zero() &&
        self.diffuse_color.is_zero() &&
        self.specular_color.is_zero())
    }

    pub fn has_reflection(&self) -> bool {
        !self.reflection_color.is_zero()
    }

    pub fn has_transmission(&self) -> bool {
        !self.transmission_color.is_zero()
    }

    pub fn calc_brdf(&self, record: &HitRecord, scene: &Scene, origin: Vector3d) -> Vector3d {
        scene.lights.iter().map(|light| {
            let light_dir = (light.position - record.point).normalized();
            let normal = record.normal.normalized();
            let viewing_dir = (origin - record.point).normalized();
            let reflection = light_dir.reflect(normal);

            let amb = self.ambient_color * light.ambient_color;

            let visible_ray = Ray::new(record.point + light_dir * 0.01, light_dir);
            let mut visible_record = HitRecord::default();
            if scene.hit(&visible_ray, 0.001, std::f64::MAX, &mut visible_record)
            {
                if (light.position - record.point).len_sqr() >= (visible_record.point - record.point).len_sqr() {
                    return amb
                }
            }
            
            let diff = Vector3d::dot(normal, light_dir).max(0.0) * self.diffuse_color * light.diffuse_color;
            let spec = Vector3d::dot(reflection, viewing_dir).max(0.0).powf(self.shininess) * self.specular_color * light.specular_color;

            amb + diff + spec
        }).fold(Vector3d::zero(), |sum, val| sum + val)
    }
}
