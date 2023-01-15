use crate::color::*;
use crate::vec::*;
use crate::utils::*;

use rand::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Diffuse(Color),
    Metal {
        color: Color,
        fuzz: f32,
    }
}

impl Material {
    pub fn scatter<R: RngCore>(&self, in_dir: Vec3, norm: Vec3, rng: &mut R) -> (Color, Vec3) {
        match self {
            Material::Diffuse(color) => (*color, random_in_hemisphere(norm, rng)),
            Material::Metal { color, fuzz } => {
                let reflected = in_dir.reflected(norm) + random_unit_vector(rng) * *fuzz;
                (*color, reflected)
            },
        }
    }
}

