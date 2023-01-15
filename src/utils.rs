use crate::vec::*;

use rand::prelude::*;



pub const EPSILON: f32 = 0.00001;

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

pub fn quadrant(v: Vec3) -> usize {
    let mut q = 0;
    for i in 0..3 {
        if v[i] < 0.0 {
            q = q | (1 << i);
        }
    }

    q
}

pub fn quadrant_dir(q: usize) -> Vec3 {
    let sign = |i| {
        if (q & ((1 << i) as usize)) != 0 {
            -1.0
        } else {
            1.0
        }
    };

    let v = Vec3::new(sign(0), sign(1), sign(2));
    debug_assert!(quadrant(v) == q);
    v
}