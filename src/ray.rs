
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

    pub fn along(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}


