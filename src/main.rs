extern crate image;
extern crate clap;

mod vector;
mod ray;
mod camera;
mod sphere;
mod scene;
mod material;

use scene::Scene;
use std::fs::File;
use std::path::Path;
use std::process;
use clap::{Arg, App};

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
    let matches = App::new("Raytracer Rust")
        .version("0.1.0")
        .author("Nicolas Hollmann")
        .about("A simple raytracer in Rust.")
        .arg(Arg::with_name("scene")
                 .required(true)
                 .takes_value(true)
                 .index(1)
                 .help("scene JSON file"))
        .arg(Arg::with_name("output")
                 .required(true)
                 .takes_value(true)
                 .index(2)
                 .default_value("rendered.png")
                 .help("filename for saving output"))
        .get_matches();

    let scene_filename = matches.value_of("scene").unwrap();
    let output_filename = matches.value_of("output").unwrap();

    let json_file_path = Path::new(&scene_filename);
    let json_file = File::open(json_file_path).unwrap_or_else(|err| {
        eprintln!("File error: {}", err);
        process::exit(1);
    });

    let scene : Scene = serde_json::from_reader(json_file).unwrap_or_else(|err| {
        eprintln!("Parsing error: {}", err);
        process::exit(1);
    });

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

    img.save(&output_filename).unwrap_or_else(|err| {
        eprintln!("Saving error: {}", err);
        process::exit(1);
    });
}
