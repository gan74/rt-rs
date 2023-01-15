use crate::color::*;
use crate::vec::*;
use crate::utils::*;

use rand::prelude::*;

use std::default::*;

use core::f32::consts::FRAC_1_PI;


#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub kind: MaterialKind,
    pub color: Color,
    pub emissive: Color,
}

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    Diffuse,
    Metal { fuzz: f32 },
}

pub struct MaterialScatter {
    pub color: Color,
    pub reflected: Vec3,
}

pub trait WithMaterial {
    fn material(&self) -> &Material;
}


impl Material {
    pub fn is_emissive(&self) -> bool {
        !self.emissive.is_zero()
    }

    pub fn scatter<R: RngCore>(&self, in_dir: Vec3, norm: Vec3, rng: &mut R) -> MaterialScatter {
        let reflected = match self.kind {
            MaterialKind::Diffuse => random_in_hemisphere(norm, rng),
            MaterialKind::Metal{fuzz} => in_dir.reflected(norm) + random_unit_vector(rng) * fuzz,
        };

        MaterialScatter {
            color: self.color, 
            reflected: reflected,
        }
    }

    pub fn eval(&self, norm: Vec3, w_in: Vec3, w_out: Vec3) -> Color {
        let refl = match self.kind {
            MaterialKind::Diffuse => {
                let cos_out = norm.dot(w_out);
                if cos_out < 0.0 || norm.dot(w_in) < 0.0 {
                    0.0
                } else {
                    FRAC_1_PI * cos_out
                }
            },
            _ => 0.0,
        };

        self.color * refl
    }
}


impl Default for Material {
    fn default() -> Self {
        Material {
            kind: MaterialKind::Diffuse,
            color: Color::from(0.5),
            emissive: Color::from(0.0),
        }
    }
}