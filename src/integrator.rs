
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

    pub fn trace<T, R, F>(scene: &T, ray: Ray, no_hit: &F, rng: &mut R, max_rays: usize) -> Color 
        where T: Hittable<Result = HitRecord>, R: RngCore, F: Fn(Ray) -> Color {

        if max_rays == 0 {
            return Color::from(0.0);
        }

        match scene.hit(ray) {
            Some(hit) => {
                let mut acc = hit.mat.map(|m| m.emissive).unwrap_or(Color::from(0.0));

                // Light contrib
                /*{
                    acc += Self::light_contrib(scene, lights, ray, &hit, rng);
                }*/

                // Material contrib
                {
                    let mat = hit.mat.unwrap_or(Material::default());
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

    /*fn light_contrib<T, R>(scene: &T, lights: &SurfaceGroup<Light>, ray: Ray, hit: &HitRecord, rng: &mut R) -> Color 
        where T: Hittable<Result = HitRecord>, R: RngCore {

        if let Some(light) = lights.sample(rng) {
            let light_sample = light.sample(rng);
            let to_light = light_sample.pos - hit.pos;
            if light_sample.norm.dot(to_light) < 0.0 {
                let mat = hit.mat.unwrap_or(default_material());

                let color = mat.eval(hit.norm, to_light.normalized(), -ray.dir);
                if !color.is_zero() && scene.hit(Ray::new_with_epsilon(hit.pos, to_light)).is_none() {
                    return light.color;
                }
            }
        }

        Color::from(0.0)
    }*/
}

