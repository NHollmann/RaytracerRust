extern crate image;

mod math;
mod camera;
mod sphere;

use math::vector::Vector3d;
use math::ray::Ray;

fn trace(ray : Ray, depth : u32) -> (f64, f64, f64) {
    let sphere = sphere::Sphere::new(Vector3d::new(0.0, 0.0, 0.0), 2.0);

    if depth > 0 && sphere.hit(&ray, 0.001, std::f64::MAX) {
        return (1.0, 1.0, 1.0);
    }

    (0.0, 0.0, 0.0)
}

fn to_color(input : f64) -> u8 {
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

    let aspect = (width as f64) / (height as f64);
    let camera = camera::Camera::new(look_from, look_at, up, fov as f64, aspect);


    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;
        let ray = camera.get_ray(u, v);

        let (r, g, b) = trace(ray, depth);
        let r = to_color(r);
        let g = to_color(g);
        let b = to_color(b);
        
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("rendered.png").unwrap();
}
