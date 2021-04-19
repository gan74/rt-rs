
use crate::vec::*;
use crate::aabb::*;
use crate::ray::*;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
}

pub struct Mesh {
    aabb: AABB,
    vertices: Vec<Vertex>,
    triangles: Vec<[u32; 3]>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<[u32; 3]>) -> Mesh {
        Mesh {
            aabb: AABB::from_points(vertices.iter().map(|v| v.pos)).unwrap(),
            vertices: vertices,
            triangles: triangles,
        }
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }
}


impl Intersectable for Mesh {
    type Result = Vec3;

    fn intersects(&self, ray: &Ray) -> Option<Self::Result> {
        if self.aabb.intersects(ray).is_none() {
            return None;
        }

        let mut hit = None;
        let mut depth_sq = f32::MAX;

        for tri in self.triangles.iter() {
            let tri =  [
                self.vertices[tri[0] as usize].pos,
                self.vertices[tri[1] as usize].pos,
                self.vertices[tri[2] as usize].pos,
            ];

            if let Some(bary) = tri.intersects(ray) {
                let pos = tri[0] * bary[0]+ tri[1] * bary[1] + tri[2] * bary[2];
                let dist_sq = ray.origin().distance2(pos);

                if dist_sq < depth_sq {
                    depth_sq = dist_sq;
                    hit = Some(pos);
                }
            }
        }

        hit
    }
}

impl Intersectable for [Vec3; 3] {
    type Result = [f32; 3];

    fn intersects(&self, ray: &Ray) -> Option<Self::Result> {
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];

        let pvec = ray.direction().cross(edge2);

        let det = edge1.dot(pvec);

        const EPSILON: f32 = 0.00001;
        if det < EPSILON/* && det > -EPSILON*/ {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.origin() - self[0];

        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(edge1);

        let v = ray.direction().dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let w = 1.0 - u - v;
        return Some([w, u, v]);
    }
}