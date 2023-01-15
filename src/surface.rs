use crate::vec::*;

use rand::prelude::*;


pub struct SurfaceSample {
    pub pos: Vec3,
    pub norm: Vec3,
}

pub trait Surface {
    fn area(&self) -> f32;

    fn sample(&self, rng: &mut dyn RngCore) -> SurfaceSample;
}

