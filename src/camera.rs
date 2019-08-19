use crate::ray::Ray;
use crate::vector::Vector3d;
use crate::scene::Scene;
use crate ::scene::HitRecord;

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

    pub fn get_pixel(&self, scene : &Scene, u : f64, v : f64) -> Vector3d {
        let ray = self.get_ray(u, v);

        trace(ray, scene.depth, &scene)
    }
}

fn trace(ray : Ray, depth : u32, scene: &Scene) -> Vector3d {
    let mut hit_record = HitRecord::default();

    if depth > 0 && scene.hit(&ray, 0.001, std::f64::MAX, &mut hit_record) {
        let mut color = Vector3d::zero();

        if hit_record.material.has_brdf() {
            color += hit_record.material.calc_brdf(&hit_record, scene, ray.get_origin());
        }

        if hit_record.material.has_reflection() {
            let reflection = Vector3d::zero() - ray.get_direction().reflect(hit_record.normal).normalized();
            let reflected_ray = Ray::new(hit_record.point + reflection * 0.001, reflection);
            let reflected_color = trace(reflected_ray, depth - 1, scene);

            color += hit_record.material.reflection_color * reflected_color;
        }

        if hit_record.material.has_transmission() {
            let transmission_ray = Ray::new(hit_record.point, hit_record.normal);
            let transmission_color = trace(transmission_ray, depth - 1, scene);

            color += hit_record.material.transmission_color * transmission_color;
        }

        return color;
    }

    Vector3d::zero()
}
