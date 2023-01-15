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

pub struct SurfaceGroup<T> {
    surfaces: Vec<(f32, Box<T>)>,
    area: f32,
}


impl<T: Surface> SurfaceGroup<T> {
    pub fn empty() -> SurfaceGroup<T> {
        SurfaceGroup {
            surfaces: Vec::new(),
            area: 0.0
        }
    }

    pub fn push(&mut self, surf: T) {
        let total_area = self.area + surf.area();

        self.surfaces.push((self.area, Box::new(surf)));
        self.area = total_area;
    }

    pub fn sample<R: RngCore>(&self, rng: &mut R) -> Option<&T> {
        if !self.surfaces.is_empty() {
          let v = rng.gen::<f32>() * self.area;

            for i in 1..self.surfaces.len() {
                if self.surfaces[i].0 > v {
                    return Some(&self.surfaces[i].1)
                }
            }
        }

        None
    }
}