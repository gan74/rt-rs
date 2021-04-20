
use crate::transform::*;
use crate::vec::*;
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
        let x = (u * 2.0 - 1.0) * self.tan_half_vfov * self.ratio;
        let y = (v * 2.0 - 1.0) * self.tan_half_vfov;

        let dir = self.right() * x + self.up() * y + self.forward();
        Ray::new(self.position(), dir)
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }

    pub fn right(&self) -> Vec3 {
        self.transform.basis()[0]
    }

    pub fn forward(&self) -> Vec3 {
        -self.transform.basis()[2]
    }

    pub fn up(&self) -> Vec3 {
        self.transform.basis()[1]
    }

    pub fn position(&self) -> Vec3 {
        self.transform.position()
    }
}