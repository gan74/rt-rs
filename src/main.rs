extern crate gltf;
extern crate image;

use std::time::Instant;

use image::ImageBuffer;

mod vec;
mod transform;
mod mesh;
mod aabb;
mod ray;
mod hit;
mod camera;
mod scene;
mod color;

use crate::scene::*;

fn main() {
    let scene = import_scene("assets/scene.gltf").expect("unable to import scene");

    let height = 512;
    let width = (height as f32 * scene.camera().ratio()) as u32;

    let start = Instant::now();
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let color = scene.trace(u, 1.0 - v);
        let norm = ((color * 0.5) + 0.5) * 255.0;
        image::Rgb([norm.r as u8, norm.g as u8, norm.b as u8])
    });

    let dur = Instant::now() - start;
    println!("{:?}", dur);


    img.save("result.png").unwrap();
}
