extern crate image;

mod math;
use math::ray::Ray;

fn trace() -> (f64, f64, f64) {
    (0.0, 0.0, 0.0)
}

fn camera_ray(u : f64, v : f64) -> Ray {
    Ray::new(Vector, direction: Vector3d)
}

fn main() {
    let width = 500;
    let height = 250;
    let depth = 30;

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let (r, g, b) = trace();
        let r = r as u8;
        let g = g as u8;
        let b = b as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    img.save("rendered.png").unwrap();
}
