use crate::color::*;
use crate::vec::*;

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




fn random_unit_vector<R: RngCore>(rng: &mut R) -> Vec3 {
    loop {
        let v = Vec3::new(
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
        );

        if v.length2() <= 1.0 {
            return v.normalized();
        }
    }
}

fn random_in_hemisphere<R: RngCore>(norm: Vec3, rng: &mut R) -> Vec3 {
    let v = random_unit_vector(rng);
    if v.dot(norm) < 0.0 {
        -v
    } else {
        v
    }
}