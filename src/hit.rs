use crate::vec::*;
use crate::ray::*;
use crate::scene::*;
use crate::material::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'hit> {
    pub dist: f32,
    pub pos: Vec3,
    pub norm: Vec3,
    pub mat: Option<&'hit Material>,
    pub obj: Option<&'hit SceneObject>,
}


pub trait Hittable {
    type Result;

    fn hit(&self, ray: Ray) -> Option<Self::Result>;
}



pub trait RefHittable {
}

impl<'h, T: 'h> RefHittable for T where &'h T: Hittable<Result = HitRecord<'h>> {
}
