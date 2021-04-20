
use crate::vec::*;
use crate::aabb::*;
use crate::ray::*;
use crate::hit::*;
use crate::material::*;

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3,
    pub norm: Vec3,
}

pub struct Mesh {
    aabb: AABB,
    vertices: Vec<Vertex>,
    triangles: Vec<[u32; 3]>,
    material: Option<Material>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<[u32; 3]>) -> Mesh {
        Mesh {
            aabb: AABB::from_points(vertices.iter().map(|v| v.pos)).unwrap(),
            vertices: vertices,
            triangles: triangles,
            material: None,
        }
    }

    pub fn new_with_material(vertices: Vec<Vertex>, triangles: Vec<[u32; 3]>, material: Material) -> Mesh {
        let mut mesh = Mesh::new(vertices, triangles);
        mesh.material = Some(material);
        mesh
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }
}


impl Hittable for Mesh {
    type Result = HitRecord;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        if self.aabb.hit(ray).is_none() {
            return None;
        }

        let mut hit: Option<HitRecord> = None;

        for ind in self.triangles.iter() {
            let tri =  [
                self.vertices[ind[0] as usize].pos,
                self.vertices[ind[1] as usize].pos,
                self.vertices[ind[2] as usize].pos,
            ];

            if let Some(bary) = tri.hit(ray) {
                let pos =
                    tri[0] * bary[0] +
                    tri[1] * bary[1] +
                    tri[2] * bary[2];

                let dist = ray.orig.distance(pos);
                if hit.is_none() || dist < hit.unwrap().dist {
                    let norm =
                        self.vertices[ind[0] as usize].norm * bary[0] +
                        self.vertices[ind[1] as usize].norm * bary[1] +
                        self.vertices[ind[2] as usize].norm * bary[2];

                    /*if norm.dot(ray.dir) < 0.0*/ {
                        hit = Some(HitRecord {
                            dist: dist,
                            pos: pos,
                            norm: norm.normalized(),
                            mat: self.material,
                        });
                    }
                }
            }
        }

        hit
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