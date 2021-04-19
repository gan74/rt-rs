use crate::vec::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub dist: f32,
    pub pos: Vec3,
    pub norm: Vec3,
}

pub trait Hittable {
    type Result;

    fn hit(&self, ray: Ray) -> Option<Self::Result>;
}




#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32
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
        })
    }
}