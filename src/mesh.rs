use crate::vec::*;
use crate::vertex::*;
use crate::aabb::*;
use crate::ray::*;
use crate::hit::*;
use crate::bvh::*;
use crate::surface::*;
use crate::material::*;
use crate::utils::*;

use rand::prelude::*;


const MAX_TRI_PER_NODE: usize = 8;


pub struct Mesh {
    bvh: Bvh<[u32; 3]>,

    triangles: Vec<[u32; 3]>,
    vertices: Vec<Vertex>,

    material: Material,

    area: f32,
    triangle_areas: Vec<f32>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, mut triangles: Vec<[u32; 3]>, material: Material) -> Mesh {
        let triangle_aabb = |tri: &[u32; 3]| Aabb::from_points(tri.iter().map(|i| vertices[*i as usize].pos)).unwrap();

        let mut mesh = Mesh {
            bvh: Bvh::new(triangles.as_mut_slice(), triangle_aabb, MAX_TRI_PER_NODE),
            triangles: triangles,
            vertices: vertices,
            material: material,
            area: 0.0,
            triangle_areas: Vec::new(),
        };

        mesh.build_surface();

        mesh
    }


    fn build_surface(&mut self) {
        let mut triangle_areas = self.triangles.iter().map(|index| {
            let p = [
                self.vertices[index[0] as usize].pos,
                self.vertices[index[1] as usize].pos,
                self.vertices[index[2] as usize].pos,
            ];
            (p[1] - p[0]).cross(p[2] - p[0]).length() * 0.5
        }).collect::<Vec<_>>();

        let mut total = 0.0;
        for a in &mut triangle_areas {
            total += *a;
            *a = total;
        }

        debug_assert!(total == triangle_areas.last().cloned().unwrap_or(0.0));

        self.area = total;
        self.triangle_areas = triangle_areas;
    }

    fn hit_triangles<'mesh>(&'mesh self, mut ray: Ray, triangles: &[[u32; 3]]) -> Option<HitRecord<'mesh>> {
        let mut hit: Option<HitRecord<'mesh>> = None;

        for index in triangles {
            let tri = [
                self.vertices[index[0] as usize],
                self.vertices[index[1] as usize],
                self.vertices[index[2] as usize],
            ];

            if let Some(bary) = tri.hit(ray) {
                let pos =
                    tri[0].pos * bary[0] +
                    tri[1].pos * bary[1] +
                    tri[2].pos * bary[2];

                let norm =
                    tri[0].norm * bary[0] +
                    tri[1].norm * bary[1] +
                    tri[2].norm * bary[2];

                let dist = ray.orig.distance(pos);

                ray = ray.with_max(dist);
                hit = Some(HitRecord {
                    dist: dist,
                    pos: pos,
                    norm: norm.normalized(),
                    mat: Some(&self.material),
                    obj: Some(self),
                });
            }
        }

        hit
    }
}



impl Surface for Mesh {
    fn area(&self) -> f32 {
        self.area
    }

    fn sample_surface(&self, rng: &mut dyn RngCore) -> SurfaceSample {
        let r = rng.gen::<f32>() * self.area;
        let i = self.triangle_areas.partition_point(|a| *a < r);

        debug_assert!(self.triangle_areas[i] >= r);

        let index = self.triangles[i];

        let tri = [
            self.vertices[index[0] as usize],
            self.vertices[index[1] as usize],
            self.vertices[index[2] as usize],
        ];


        let xi_u = rng.gen::<f32>();
        let xi_v = rng.gen::<f32>();

        let inv_sqr_u = (1.0 - xi_u).sqrt();
        let alpha = 1.0 - inv_sqr_u;
        let beta = xi_v * inv_sqr_u;
        let gamma = 1.0 - alpha - beta;

        let bary = [alpha, beta, gamma];
 
        let pos =
            tri[0].pos * bary[0] +
            tri[1].pos * bary[1] +
            tri[2].pos * bary[2];

        let norm =
            tri[0].norm * bary[0] +
            tri[1].norm * bary[1] +
            tri[2].norm * bary[2];

        SurfaceSample {
            pos: pos,
            norm: norm.normalized(),
        }
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

impl<'mesh> Hittable for &'mesh Mesh {
    type Result = HitRecord<'mesh>;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        self.bvh.trace(ray, |r, tris| self.hit_triangles(r, tris))
    }
}


impl Hittable for [Vertex; 3] {
    type Result = [f32; 3];

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        [self[0].pos, self[1].pos, self[2].pos].hit(ray)
    }
}

impl Hittable for [Vec3; 3] {
    type Result = [f32; 3];

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        let edge1 = self[1] - self[0];
        let edge2 = self[2] - self[0];

        let pvec = ray.dir.cross(edge2);

        let det = edge1.dot(pvec);

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