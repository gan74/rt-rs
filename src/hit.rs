use crate::vec::*;
use crate::ray::*;
use crate::material::*;

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub dist: f32,
    pub pos: Vec3,
    pub norm: Vec3,
    pub mat: Option<Material>,
}

#[derive(Debug, Clone, Copy)]
pub struct HitRecordRef<'scene> {
    pub dist: f32,
    pub pos: Vec3,
    pub norm: Vec3,
    pub mat: Option<&'scene Material>,
}


pub trait Hittable {
    type Result;

    fn hit(&self, ray: Ray) -> Option<Self::Result>;
}

pub trait Hit {
    fn position(&self) -> Vec3;
    fn normal(&self) -> Vec3;
    fn distance(&self) -> f32;
}


impl Hit for HitRecord {
    fn position(&self) -> Vec3 {
        self.pos
    }

    fn normal(&self) -> Vec3 {
        self.norm
    }

    fn distance(&self) -> f32 {
        self.dist
    }
}

impl<'scene> Hit for HitRecordRef<'scene> {
    fn position(&self) -> Vec3 {
        self.pos
    }

    fn normal(&self) -> Vec3 {
        self.norm
    }

    fn distance(&self) -> f32 {
        self.dist
    }
}



impl<'scene> From<HitRecordRef<'scene>> for HitRecord {
    fn from(hit: HitRecordRef<'scene>) -> HitRecord {
        HitRecord {
            dist: hit.dist,
            pos: hit.pos,
            norm: hit.norm,
            mat: hit.mat.cloned(),
        }
    }
}
