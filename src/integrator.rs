
use crate::ray::*;
use crate::hit::*;
use crate::camera::*;
use crate::color::*;
use crate::material::*;

use rand::prelude::*;

pub struct Integrator {
}

impl Integrator {
	pub fn generate_ray<R: RngCore>(camera: &Camera, x: u32, y: u32, width: u32, height: u32, rng: &mut R) -> Ray {
        let x = x as f32;
        let y = y as f32;
        let width = width as f32;
        let height = height as f32;

        let (x_min, x_max) = (x / width, (x + 1.0) / width);
        let (y_min, y_max) = (y / height, (y + 1.0) / height);

        let u = rng.gen::<f32>() * (x_max - x_min) + x_min;
        let v = rng.gen::<f32>() * (y_max - y_min) + y_min;

        camera.generate_ray(u, 1.0 - v)
    }

    pub fn trace<T: Hittable<Result = HitRecord>, R: RngCore>(scene: &T, ray: Ray, rng: &mut R, max_rays: usize) -> Color {
        if max_rays == 0 {
            return Color::from(0.0);
        }

        let default_mat = Material::Diffuse(Color::from(0.5));

        match scene.hit(ray) {
            Some(hit) => {
                let mat = hit.mat.unwrap_or(default_mat);
                let (color, new_dir) = mat.scatter(ray.dir, hit.norm, rng);
                Self::trace(scene, Ray::new_with_epsilon(hit.pos, new_dir), rng, max_rays - 1) * color
            },

            None => {
                let v = (ray.dir.y + 1.0) * 0.5;
                Color::new(0.5, 0.7, 1.0) * v + (1.0 - v)
            },
        }
    }
}





