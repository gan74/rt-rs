extern crate gltf;
extern crate image;
extern crate rand;
extern crate rayon;

use std::time::Instant;

use image::ImageBuffer;
use rayon::prelude::*;

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


    let spp = 16;

    let start = Instant::now();


    let data: Vec<_> = (0..width * height).into_par_iter().map(|i| {
        let u = (i % width) as f32 / width as f32;
        let v = (i / width) as f32 / height as f32;

        let mut rng = rand::thread_rng();

        // TODO randomize ray gen too
        let ray = scene.generate_ray(u, 1.0 - v);

        let mut color = Color::from(0.0);
        for _ in 0..spp {
            color = color + scene.trace(ray, &mut rng, 5);
        }

        color / spp as f32
    }).collect();

    println!("Done in {:?}", Instant::now() - start);


    let img = ImageBuffer::from_fn(width, height, |x, y| image::Rgb(data[(y * width + x) as usize].to_srgb()) );
    img.save("result.png").unwrap();


}
