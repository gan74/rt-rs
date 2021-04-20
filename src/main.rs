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

use crate::scene::*;
use crate::color::*;


#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = import_scene("assets/scene.gltf").expect("unable to import scene");
    let height = 512;
    let width = (height as f32 * scene.camera().ratio()) as u32;

    let spp = 16;

    let start = Instant::now();
    let data: Vec<_> = (0..width * height).into_par_iter().map(|i| {
        let mut rng = rand::thread_rng();
        let ray = scene.generate_ray(&mut rng, i % width, i / width, width, height);

        let mut color = Color::from(0.0);
        for _ in 0..spp {
            color = color + scene.trace(ray, &mut rng, 5);
        }

        (color / spp as f32).to_srgb()
    }).collect();
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