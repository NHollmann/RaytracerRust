use crate::math::vector::Vector3d;
use crate::math::ray::Ray;
use crate::material::Material;
use crate::sphere::Sphere;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vector3d,
    pub normal: Vector3d,
    pub material: Material,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            point: Vector3d::zero(),
            normal: Vector3d::zero(),
            material: Material::empty(),
        }
    }
}

pub struct Light {
    pub position: Vector3d,
    pub ambient_color: Vector3d,
    pub diffuse_color: Vector3d,
    pub specular_color: Vector3d,
}

pub struct Scene {
    pub objects: Vec<Sphere>,
    pub lights: Vec<Light>,
    pub camera_pos : Vector3d,
}

impl Scene {
    pub fn hit(&self, ray : &Ray, t_min : f64, t_max : f64, record : &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest = t_max;

        for object in &self.objects {
            let mut iter_record = HitRecord::default();
            if object.hit(ray, t_min, closest, &mut iter_record) {
                hit_anything = true;
                closest = iter_record.t;
                record.t = iter_record.t;
                record.point = iter_record.point;
                record.normal = iter_record.normal;
                record.material = iter_record.material;
            }
        }

        hit_anything
    }
}
