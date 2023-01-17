use crate::color::*;
use crate::vec::*;
use crate::utils::*;

use rand::prelude::*;

use std::default::*;


#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub roughness: f32,
    pub metallic: f32,
    pub color: Color,
    pub emissive: Color,
}

pub struct MaterialSample {
    pub color: Color,
    pub reflected_dir: Vec3,
}


impl Material {
    pub fn is_emissive(&self) -> bool {
        !self.emissive.is_zero()
    }

    pub fn pdf(&self, norm: Vec3, in_dir: Vec3, out_dir: Vec3) -> f32 {
        let cos_theta = norm.dot(out_dir);
        if cos_theta <= 0.0 || norm.dot(in_dir) <= 0.0 { 
            return 0.0;
        }
        cos_theta * INV_PI
    }

    pub fn sample<R: RngCore>(&self, _in_dir: Vec3, norm: Vec3, rng: &mut R) -> MaterialSample {
        // TODO: importance sampling
        let reflected_dir = random_in_hemisphere(norm, rng);
        MaterialSample {
            color: self.color * INV_PI,
            reflected_dir: reflected_dir,
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