extern crate image;

mod math;
mod camera;
mod sphere;
mod scene;
mod material;

use math::vector::Vector3d;
use math::ray::Ray;
use scene::Scene;
use std::env;
use std::fs::File;
use std::path::Path;

fn trace(ray : Ray, depth : u32, scene : &Scene) -> Vector3d {
    let mut hit_record = scene::HitRecord::default();

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
    let args: Vec<String> = env::args().collect();

    if env::args().count() < 3 {
        panic!("Too few arguments!");
    }

    let json_file_path = Path::new(&args[1]);
    let json_file = File::open(json_file_path).expect("Error: file not found!");
    let scene : Scene = serde_json::from_reader(json_file).expect("Error on parsing file!");

    let width = scene.width;
    let height = scene.height;
    let depth = scene.depth;
    let fov = scene.fov;

    let aspect = (width as f64) / (height as f64);
    let camera = camera::Camera::new(scene.look_from, scene.look_at, scene.up, fov as f64, aspect);

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let ray = camera.get_ray(1.0 - u, 1.0 - v);

        let color = trace(ray, depth, &scene);
        let r = to_color(color.x);
        let g = to_color(color.y);
        let b = to_color(color.z);
        
        *pixel = image::Rgb([r, g, b]);
    }

    img.save(&args[2]).unwrap();
}
