use crate::vec::*;

use rand::prelude::*;


pub fn random_unit_vector<R: RngCore>(rng: &mut R) -> Vec3 {
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

pub fn random_in_hemisphere<R: RngCore>(norm: Vec3, rng: &mut R) -> Vec3 {
    let v = random_unit_vector(rng);
    if v.dot(norm) < 0.0 {
        -v
    } else {
        v
    }
}

pub fn main_axis(v: Vec3) -> usize {
    let (mut axis, mut value) = (0, v.x.abs());
    for i in 1..3 {
        if v[i].abs() > value {
            axis = if v[i] < 0.0 {
                i + 3
            } else {
                i
            };
            value = v[i].abs();
        }
    }

    axis
}