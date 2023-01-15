
use crate::vec::*;
use crate::vertex::*;
use crate::aabb::*;
use crate::ray::*;
use crate::hit::*;
use crate::bvh::*;
use crate::material::*;


const MAX_TRI_PER_NODE: usize = 8;

pub struct Mesh {
    bvh: Bvh<[u32; 3]>,
    vertices: Vec<Vertex>,
    material: Material,
}


impl Mesh {
    pub fn new(vertices: Vec<Vertex>, mut triangles: Vec<[u32; 3]>, material: Material) -> Mesh {
        let triangle_aabb = |tri: &[u32; 3]| Aabb::from_points(tri.iter().map(|i| vertices[*i as usize].pos)).unwrap();
        Mesh {
            bvh: Bvh::new(triangles.as_mut_slice(), triangle_aabb, MAX_TRI_PER_NODE),
            vertices: vertices,
            material: material,
        }
    }


    fn hit_triangles(&self, mut ray: Ray, triangles: &[[u32; 3]]) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for index in triangles {
            let tri =  [
                self.vertices[index[0] as usize].pos,
                self.vertices[index[1] as usize].pos,
                self.vertices[index[2] as usize].pos,
            ];

            if let Some(bary) = tri.hit(ray) {
                let pos =
                    tri[0] * bary[0] +
                    tri[1] * bary[1] +
                    tri[2] * bary[2];

                let norm =
                    self.vertices[index[0] as usize].norm * bary[0] +
                    self.vertices[index[1] as usize].norm * bary[1] +
                    self.vertices[index[2] as usize].norm * bary[2];

                let dist = ray.orig.distance(pos);

                ray = ray.with_max(dist);
                hit = Some(HitRecord {
                    dist: dist,
                    pos: pos,
                    norm: norm.normalized(),
                    mat: Some(self.material),
                });
            }
        }

        hit
    }
}

impl WithAabb for Mesh {
    fn aabb(&self) -> Aabb {
        self.bvh.aabb()
    }
}

impl WithMaterial for Mesh {
    fn material(&self) -> &Material {
        &self.material
    }
}

impl Hittable for Mesh {
    type Result = HitRecord;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        self.bvh.trace(ray, |r, tris| self.hit_triangles(r, tris))
    }
}


impl Hittable for [Vec3; 3] {
    type Result = [f32; 3];

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];

        let pvec = ray.dir.cross(edge2);

        let det = edge1.dot(pvec);

        const EPSILON: f32 = 0.00001;
        if det < EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        let tvec = ray.orig - self[0];

        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(edge1);

        let v = ray.dir.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = edge2.dot(qvec) * inv_det;
        if t < 0.0 || t > ray.max {
            return None;
        }

        let w = 1.0 - u - v;
        return Some([w, u, v]);
    }
}