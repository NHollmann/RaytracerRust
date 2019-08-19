extern crate image;

mod vector;
mod ray;
mod camera;
mod sphere;
mod scene;
mod material;

use scene::Scene;
use std::env;
use std::fs::File;
use std::path::Path;

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

    let aspect = (width as f64) / (height as f64);
    let camera = camera::Camera::new(scene.look_from, scene.look_at, scene.up, scene.fov as f64, aspect);

    let mut img = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let u = x as f64 / width as f64;
        let v = y as f64 / height as f64;

        let color = camera.get_pixel(&scene, 1.0 - u, 1.0 - v);
        let r = to_color(color.x);
        let g = to_color(color.y);
        let b = to_color(color.z);
        
        *pixel = image::Rgb([r, g, b]);
    }

    img.save(&args[2]).unwrap();
}
