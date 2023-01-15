use crate::color::*;
use crate::surface::*;

use rand::prelude::*;


pub struct Light {
    pub color: Color,

    surface: Box<dyn Surface + Sync>,
}


impl Light {
    pub fn new(color: Color, surf: Box<dyn Surface + Sync>) -> Light {
        Light {
            color: color,
            surface: surf,
        }
    }
}

impl Surface for Light {
    fn area(&self) -> f32 {
        self.surface.area()
    }

    fn sample(&self, rng: &mut dyn RngCore) -> SurfaceSample {
        self.surface.sample(rng)
    }
}