use crate::vec::*;
use crate::utils::*;

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
        Ray::new_with_offset(orig, dir, EPSILON)
    }

    pub fn new_with_offset(orig: Vec3, dir: Vec3, offset: f32) -> Ray {
        let ray = Ray::new(orig, dir);
        Ray {
            orig: ray.orig + ray.dir * offset,
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


