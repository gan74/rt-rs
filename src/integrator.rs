
use crate::ray::*;
use crate::hit::*;
use crate::scene::*;
use crate::camera::*;
use crate::color::*;
use crate::surface::*;

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

    pub fn trace<R: RngCore, F: Fn(Ray) -> Color>(scene: &Scene, ray: Ray, no_hit: &F, rng: &mut R, max_rays: usize) -> Color {
        if max_rays == 0 {
            return Color::from(0.0);
        }

        match scene.hit(ray) {
            Some(hit) => {
                let mut acc = hit.mat.map(|m| m.emissive).unwrap_or(Color::from(0.0));

                // Light contrib
                {
                    acc += Self::light_contrib(scene, ray, &hit, rng);
                }

                // Material contrib
                if let Some(mat) = hit.mat {
                    let scattered = mat.scatter(ray.dir, hit.norm, rng);

                    if !scattered.color.is_zero() {
                        acc += Self::trace(scene, Ray::new_with_epsilon(hit.pos, scattered.reflected), no_hit, rng, max_rays - 1) * scattered.color;
                    }
                }

                acc
            },

            None => no_hit(ray),
        }
    }

    fn light_contrib<R: RngCore>(scene: &Scene, ray: Ray, hit: &HitRecord, rng: &mut R) -> Color {
        if let Some(mat) = hit.mat {
            if let Some((emitter, radiance)) = scene.sample_emitter_surface(rng) {
                let sample = emitter.sample_surface(rng);
                let shadow_ray_dir = (sample.pos - hit.pos).normalized();

                /*if sample.norm.dot(shadow_ray_dir) < 0.0 {
                    return Color::from(0.0);
                }*/

                let refl = mat.eval(hit.norm, shadow_ray_dir, -ray.dir);
                if refl.is_zero() {
                    return Color::from(0.0);
                }

                if let Some(shadow_hit) = scene.hit(Ray::new_with_epsilon(hit.pos, shadow_ray_dir)) {
                    if let Some(occluder) = shadow_hit.obj {
                        if std::ptr::eq(occluder, emitter) {
                            return refl * radiance;
                        }
                    }
                }
            }
        }

        Color::from(0.0)
    }
}

