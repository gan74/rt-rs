extern crate gltf;
extern crate image;
extern crate rand;

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
use crate::color::*;

fn main() {
    let scene = import_scene("assets/scene.gltf").expect("unable to import scene");

    let height = 512;
    let width = (height as f32 * scene.camera().ratio()) as u32;

    let mut rng = rand::thread_rng();

    let spp = 1;

    let start = Instant::now();
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        // TODO randomize ray gen too
        let ray = scene.generate_ray(u, 1.0 - v);

        let mut color = Color::from(0.0);
        for _ in 0..spp {
            color = color + scene.trace(ray, &mut rng, 5);
        }

        image::Rgb((color / spp as f32).to_srgb())
    });

    let dur = Instant::now() - start;
    println!("{:?}", dur);


    img.save("result.png").unwrap();
}
