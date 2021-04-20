
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
    pub max: f32
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir.normalized(),
            max: f32::MAX,
        }
    }

    pub fn new_with_epsilon(orig: Vec3, dir: Vec3) -> Ray {
        let ray = Ray::new(orig, dir);
        Ray {
            orig: ray.orig + ray.dir * 0.00001,
            dir: ray.dir,
            max: f32::MAX,
        }
    }

    pub fn along(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }

    pub fn with_max(self, max: f32) -> Ray {
        Ray {
            orig: self.orig,
            dir: self.dir,
            max: max,
        }
    }
}


