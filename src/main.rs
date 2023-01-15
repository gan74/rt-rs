#![allow(dead_code)]

extern crate gltf;
extern crate rand;
extern crate rayon;
extern crate show_image;

use std::time::Instant;

use show_image::{ImageView, ImageInfo, WindowOptions, create_window, event};
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
mod material;
mod integrator;
mod bvh;
mod vertex;
mod surface;
mod shapes;
mod utils;


use crate::scene::*;
use crate::ray::*;
use crate::color::*;
use crate::integrator::*;


const SPP: usize = 16;
const MAX_BOUNCES: usize = 5;


#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let scene = import_scene("assets/scene.gltf").expect("unable to import scene");
    let camera = scene.camera();

    println!("Loaded in {:?}", (Instant::now() - start));

    let height = 512;
    let width = (height as f32 * camera.ratio()) as u32;

    let start = Instant::now();

    let data = (0..width * height).into_par_iter().map(|i| {
        let mut rng = rand::thread_rng();

        let no_hit = |ray: Ray| {
            let v = (ray.dir.y + 1.0) * 0.5;
            Color::new(0.5, 0.7, 1.0) * v + (1.0 - v)
        };


        let no_hit = |_: Ray| Color::from(0.0);

        let mut color = Color::from(0.0);
        for _ in 0..SPP {
            let ray = Integrator::generate_ray(&camera, i % width, i / width, width, height, &mut rng);
            color = color + Integrator::trace(&scene, ray, &no_hit, &mut rng, MAX_BOUNCES);
        }

        assert!(color.r >= 0.0 && color.g >= 0.0 && color.b >= 0.0);
        (color / SPP as f32).to_srgb()
    }).collect::<Vec<_>>();

    let duration = Instant::now() - start;
    println!("Done in {:?}", duration);
    println!("{:.2} MS/s", ((width * height) as usize * SPP) as f64 / 1_000_000.0 / duration.as_secs_f64());


    {
        let mut pixel_data = Vec::with_capacity(data.len() * 3);
        for rgb in data {
            pixel_data.push(rgb[0]);
            pixel_data.push(rgb[1]);
            pixel_data.push(rgb[2]);
        }

        let options = WindowOptions::default()
            .set_size([width, height])
            .set_resizable(false);
        let window = create_window("Result", options).unwrap();
        window.set_image("result", ImageView::new(ImageInfo::rgb8(width, height), pixel_data.as_slice())).unwrap();

        for event in window.event_channel()? {
            if let event::WindowEvent::KeyboardInput(event) = event {
                if event.input.key_code == Some(event::VirtualKeyCode::Escape) && event.input.state.is_pressed() {
                    break;
                }
            }
        }
    }

    Ok(())
}