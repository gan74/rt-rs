use crate::color::*;
use crate::vec::*;
use crate::utils::*;

use rand::prelude::*;

use std::default::*;

use core::f32::consts::FRAC_1_PI;


#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub roughness: f32,
    pub metallic: f32,
    pub color: Color,
    pub emissive: Color,
}

pub struct MaterialScatter {
    pub color: Color,
    pub reflected: Vec3,
}


impl Material {
    pub fn is_emissive(&self) -> bool {
        !self.emissive.is_zero()
    }

    pub fn pdf(&self, norm: Vec3, in_dir: Vec3, out_dir: Vec3) -> f32 {
        let cos_out = norm.dot(out_dir);
        let cos_in = norm.dot(in_dir);
        if cos_out < 0.0 || cos_in < 0.0 { 
            return 0.0;
        }
        FRAC_1_PI * cos_out
    }

    pub fn scatter<R: RngCore>(&self, _in_dir: Vec3, norm: Vec3, rng: &mut R) -> MaterialScatter {
        // TODO: importance sampling
        let refl = random_in_hemisphere(norm, rng);
        MaterialScatter {
            color: self.color,
            reflected: refl,
        }
    }

    pub fn eval(&self, norm: Vec3, in_dir: Vec3, out_dir: Vec3) -> Color {
        self.color * self.pdf(norm, in_dir, out_dir)
    }
}


impl Default for Material {
    fn default() -> Self {
        Material {
            roughness: 0.5,
            metallic: 0.0,
            color: Color::from(0.75),
            emissive: Color::from(0.0),
        }
    }
}