
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Ray {
        Ray {
            orig: orig,
            dir: dir.normalized(),
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    pub fn along(&self, t: f32) -> Vec3 {
        self.orig + self.dir * t
    }
}


pub fn intersect(ray: &Ray, tri: [Vec3; 3]) -> Option<(f32, f32, f32)> {
    let edge_1 = tri[1] - tri[0];
    let edge_2 = tri[2] - tri[0];

    let p_vec = ray.dir.cross(edge_2);

    let det = edge_1.dot(p_vec);

    const EPSILON: f32 = 0.00001;
    if det < EPSILON/* && det > -EPSILON*/ {
        return None;
    }

    let inv_det = 1.0 / det;

    let t_vec = ray.orig - tri[0];

    let u = t_vec.dot(p_vec) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let q_vec = t_vec.cross(edge_1);

    let v = ray.dir.dot(q_vec) * inv_det;
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    let w = 1.0 - u - v;
    return Some((w, u, v));
}

/*pub fn intersection_pos(ray: &Ray, tri: [Vec3; 3]) -> Option<Vec3> {
    intersect(ray, tri).map(|bary|
        tri[0] * bary.0 +
        tri[1] * bary.1 +
        tri[2] * bary.2
    )
}*/