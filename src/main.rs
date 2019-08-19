extern crate image;

mod math;
mod camera;
mod sphere;
mod scene;
mod material;

use math::vector::Vector3d;
use math::ray::Ray;
use scene::Scene;
use scene::Light;

fn trace(ray : Ray, depth : u32, scene : &Scene) -> Vector3d {
    let mut hit_record = scene::HitRecord::default();

    if depth > 0 && scene.hit(&ray, 0.001, std::f64::MAX, &mut hit_record) {
        let mut color = Vector3d::zero();

        if hit_record.material.has_brdf() {
            color += hit_record.material.calc_brdf(&hit_record, scene);
        }

        if hit_record.material.has_reflection() {
            let reflected_ray = Ray::new(hit_record.point, hit_record.normal);
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

fn to_color(input : f64) -> u8 {
    if input >= 1.0 {
        return 255
    }
    if input <= 0.0 {
        return 0
    }
    (input * 255.99) as u8
}

fn main() {
    let width = 500;
    let height = 250;
    let depth = 30;
    let fov = 90;

    let look_from = Vector3d::new(0.0, 0.0, -5.0);
    let look_at = Vector3d::new(0.0, 0.0, 0.0);
    let up = Vector3d::new(0.0, 1.0, 0.0);

    let sphere1 = sphere::Sphere::new(
        Vector3d::new(0.0, 0.0, 0.0), 
        2.0,
        material::Material::new_specular(
            Vector3d::one(), 
            Vector3d::one(),
            Vector3d::one(),
            10.0
        )
    );

    let sphere2 = sphere::Sphere::new(
        Vector3d::new(0.5, 1.0, -2.5), 
        0.25,
        material::Material::new_specular(
            Vector3d::one(), 
            Vector3d::one(),
            Vector3d::one(),
            10.0
        )
    );

    let light = Light {
        position: Vector3d::new(1.0, 2.0, -5.0),
        ambient_color: Vector3d::new(0.1, 0.0, 0.3),
        diffuse_color: Vector3d::new(1.0, 1.0, 0.0),
        specular_color: Vector3d::new(0.0, 0.0, 1.0),
    };

    let scene = Scene {
        objects: vec![sphere1, sphere2],
        lights: vec![light],
        camera_pos: look_from
    };
    

    let aspect = (width as f64) / (height as f64);
    let camera = camera::Camera::new(look_from, look_at, up, fov as f64, aspect);

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let ray = camera.get_ray(u, v);

        let color = trace(ray, depth, &scene);
        let r = to_color(color.x);
        let g = to_color(color.y);
        let b = to_color(color.z);
        
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("rendered.png").unwrap();
}
