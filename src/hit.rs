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

pub trait Hittable {
    type Result;

    fn hit(&self, ray: Ray) -> Option<Self::Result>;
}
