use crate::vec::*;
use crate::hit::*;
use crate::ray::*;
use crate::surface::*;

use rand::prelude::*;


use core::f32::consts::PI;


#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hittable for Sphere {
    type Result = HitRecord;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        let dir = ray.orig - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = ray.dir.dot(dir);
        let c = dir.dot(dir) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrt_dis = discriminant.sqrt();

        let mut r = -b - sqrt_dis;
        if r < 0.0 {
            r = -b + sqrt_dis;
        }

        let dist = r / a;
        let hit_pos = ray.along(dist);

        Some(HitRecord {
            dist: dist,
            pos: hit_pos,
            norm: (hit_pos - self.center).normalized(),
            mat: None,
        })
    }
}

impl Surface for Sphere {
    fn area(&self) -> f32 {
        self.radius * self.radius * PI * 4.0 
    }

    fn sample(&self, rng: &mut dyn RngCore) -> SurfaceSample {
        // https://math.stackexchange.com/questions/1585975/how-to-generate-random-points-on-a-sphere
        let u = rng.gen::<f32>();
        let v = rng.gen::<f32>();
        let lat = (2.0 * u).acos() - (PI * 0.5);
        let long = v * 2.0 * PI;
        let cos_lat = lat.cos();

        let p = Vec3::new(cos_lat * long.cos(), cos_lat * long.sin(), lat.sin());
        SurfaceSample {
            pos: self.center + p * self.radius,
            norm: p,
        }
    }
}