use crate::vec::*;
use crate::ray::*;
use crate::scene::*;
use crate::material::*;

#[derive(Clone, Copy)]
pub struct HitRecord<'hit> {
    pub dist: f32,
    pub pos: Vec3,
    pub norm: Vec3,
    pub ray: Ray,
    pub obj: Option<&'hit SceneObject>,
}


pub trait Hittable {
    type Result;

    fn hit(&self, ray: Ray) -> Option<Self::Result>;
}



impl<'hit> HitRecord<'hit> {
    pub fn material(&self) -> Option<&'hit Material> {
        self.obj.map(|o| o.material())
    }
}



pub trait RefHittable {
}

impl<'h, T: 'h> RefHittable for T where &'h T: Hittable<Result = HitRecord<'h>> {
}
