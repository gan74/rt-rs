
use crate::vec::*;

pub trait Intersectable {
    type Result;

    fn intersects(&self, ray: &Ray) -> Option<Self::Result>;
}


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir.normalized(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn along(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}
