extern crate gltf;
extern crate image;

use std::time::{Duration, Instant};

use image::ImageBuffer;

mod vec;
mod transform;
mod mesh;
mod aabb;
mod ray;
mod scene;

use crate::scene::*;

fn main() {
    let scene = import_scene("assets/scene.gltf").expect("unable to import scene");

    let height = 512;
    let width = (height as f32 * scene.camera().ratio()) as u32;

    let start = Instant::now();
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let u = x as f32 / width as f32;
        let v = y as f32 / height as f32;

        let hit = scene.trace_ray(u, 1.0 - v);

        /*let color = if let Some(hit_pos) = hit {
            let norm = ((hit_pos * 0.5) + 0.5) * 255.0;
            [norm.x as u8, norm.y as u8, norm.z as u8]
        } else {
            [0u8; 3]
        };

        image::Rgb(color)*/

        let depth = match hit {
            Some(pos) => {
                let dist = (scene.camera().transform().position() - pos).length();
                1.0 / (dist + 1.0)
            },

            None => 0.0
        };

        let depth = (depth.powf(1.0 / 2.2) * 256.0).min(255.0) as u8;
        image::Luma([depth])
    });

    let dur = Instant::now() - start;
    println!("{:?}", dur);


    img.save("result.png").unwrap();
}
