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

use crate::scene::*;
use crate::color::*;
use crate::integrator::*;

const SPP: usize = 16;

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = import_scene("assets/buggy.gltf").expect("unable to import scene");
    let camera = scene.camera();

    let height = 512;
    let width = (height as f32 * camera.ratio()) as u32;

    let start = Instant::now();

    let data = (0..width * height).into_par_iter().map(|i| {
        let mut rng = rand::thread_rng();

        let mut color = Color::from(0.0);
        for _ in 0..SPP {
            let ray = Integrator::generate_ray(&camera, i % width, i / width, width, height, &mut rng);
            color = color + Integrator::trace(&scene, ray, &mut rng, 5);
        }

        (color / SPP as f32).to_srgb()
    }).collect::<Vec<_>>();

    println!("Done in {:?}", Instant::now() - start);


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