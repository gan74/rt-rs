
use crate::transform::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    transform: Transform,
    tan_half_vfov: f32,
    ratio: f32,
}

impl Camera {
    pub fn new(tr: Transform, vfov: f32, ratio: f32) -> Camera {
        Camera {
            transform: tr,
            tan_half_vfov: (vfov * 0.5).tan(),
            ratio: ratio,
        }
    }

    pub fn generate_ray(&self, u: f32, v: f32) -> Ray {
        let x = (u * 2.0 - 1.0) * self.tan_half_vfov;
        let y = (v * 2.0 - 1.0) * self.tan_half_vfov;

        let dir = self.transform.right() * x + self.transform.up() * y + self.transform.forward();
        Ray::new(self.transform.position(), dir)
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }
}