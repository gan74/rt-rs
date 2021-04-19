
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir.normalized(),
        }
    }

    pub fn new_with_epsilon(orig: Vec3, dir: Vec3) -> Ray {
        let ray = Ray::new(orig, dir);
        Ray {
            orig: ray.orig + ray.dir * 0.00001,
            dir: ray.dir,
        }
    }


    pub fn along(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}


