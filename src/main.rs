#![allow(dead_code)]


extern crate gltf;
extern crate rand;
extern crate rayon;
extern crate show_image;


use std::time::Instant;
use std::path::Path;


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
mod utils;
mod denoise;
mod image;


use crate::scene::*;
use crate::ray::*;
use crate::color::*;
use crate::integrator::*;
use crate::image::*;



const SPP: usize = 16;
const MAX_BOUNCES: usize = 4;
const SCENE_FILE: &str = "assets/cornel.gltf";



fn load() -> Scene {
    let start = Instant::now();

    let scene_path = Path::new(SCENE_FILE);
    let scene = import_scene(scene_path).expect("unable to import scene");

    println!("Loaded in {:?}", (Instant::now() - start));

    scene
}

fn trace(scene: &Scene) -> Image {
    let start = Instant::now();

    let camera = scene.camera();

    let height = 768;
    let width = (height as f32 * camera.ratio()) as u32;

    let image = Image::new(width, height, (0..width * height).into_par_iter().map(|i| {
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

        color / SPP as f32
    }).collect::<Vec<_>>());

    let duration = Instant::now() - start;
    println!("Traced in {:?}", duration);
    println!("{:.2} MS/s", ((width * height) as usize * SPP) as f64 / 1_000_000.0 / duration.as_secs_f64());

    image
}

fn scene_name() -> String {
    let scene_path = Path::new(SCENE_FILE);
    let scene_name = String::from(Path::new(scene_path.file_name().unwrap()).file_stem().unwrap().to_str().unwrap());
    format!("{} ({}spp)", scene_name, SPP)
}

fn srgb_data(image: &Image) -> Vec<u8> {
    let mut pixel_data = Vec::with_capacity(image.pixel_count() * 3);
    for rgb in image.pixels() {
        let srgb = rgb.to_srgb();
        pixel_data.push(srgb.r);
        pixel_data.push(srgb.g);
        pixel_data.push(srgb.b);
    }

    pixel_data
}



#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = load();
    let image = trace(&scene);
    //let image = denoise(&image);

    {
        let name = scene_name();

        let options = WindowOptions::default()
            .set_size([image.width(), image.height()])
            .set_resizable(false)
            .set_default_controls(false);

        let window = create_window(name.clone(), options).unwrap();

        let pixel_data = srgb_data(&image);
        window.set_image(name, ImageView::new(ImageInfo::rgb8(image.width(), image.height()), pixel_data.as_slice())).unwrap();

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