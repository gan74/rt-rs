
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
    bvh: BvhNode,
    vertices: Vec<Vertex>,
    material: Option<Material>,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, triangles: Vec<[u32; 3]>) -> Mesh {
        Mesh {
            //triangles: triangles.clone(),
            bvh: BvhNode::new(&vertices, triangles),
            vertices: vertices,
            material: None,
        }
    }

    pub fn new_with_material(vertices: Vec<Vertex>, triangles: Vec<[u32; 3]>, material: Material) -> Mesh {
        let mut mesh = Mesh::new(vertices, triangles);
        mesh.material = Some(material);
        mesh
    }

    pub fn aabb(&self) -> Aabb {
        self.bvh.aabb
    }



    fn hit_triangles(&self, triangles: &[[u32; 3]], mut ray: Ray) -> Option<HitRecord> {
        let mut hit: Option<HitRecord> = None;

        for index in triangles.iter() {
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
                    mat: self.material,
                });
            }
        }

        hit
    }

    fn hit_bvh_node(&self, node: &BvhNode, mut ray: Ray) -> Option<HitRecord> {
        if node.aabb.hit(ray).is_none() {
            return None;
        }
        match &node.content {
            BvhContent::Leaf(tris) => self.hit_triangles(&tris, ray),
            BvhContent::Node(children) => {
                let dist_sq = |c: &BvhNode| c.aabb.center().distance2(ray.orig);

                let children = if dist_sq(&children.0) < dist_sq(&children.1) {
                    [&children.0, &children.1]
                } else {
                    [&children.1, &children.0]
                };

                let mut hit_rec: Option<HitRecord> = None;
                for child in children.iter() {
                    if let Some(hit) = self.hit_bvh_node(child, ray) {
                        ray = ray.with_max(hit.dist);
                        hit_rec = Some(hit);
                    }
                }
                hit_rec
            },
        }
    }
}


impl Hittable for Mesh {
    type Result = HitRecord;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        self.hit_bvh_node(&self.bvh, ray)
    }
}


struct BvhNode {
    aabb: Aabb,
    content: BvhContent,
}

enum BvhContent {
    Leaf(Vec<[u32; 3]>),
    Node(Box<(BvhNode, BvhNode)>),
}

impl BvhNode {
    fn new(vertices: &[Vertex], mut triangles: Vec<[u32; 3]>) -> BvhNode {
        BvhNode::build(vertices, &mut triangles, 0)
    }

    fn build(vertices: &[Vertex], triangles: &mut [[u32; 3]], axis: usize) -> BvhNode {
        if triangles.len() < 32 {
            return BvhNode {
                aabb: Aabb::from_points(triangles.iter().flatten().map(|i| vertices[*i as usize].pos)).unwrap(),
                content: BvhContent::Leaf(Vec::from(triangles)),
            };
        }

        let on_axis = |tri: &[u32; 3]| {
            vertices[tri[0] as usize].pos[axis] +
            vertices[tri[1] as usize].pos[axis] +
            vertices[tri[2] as usize].pos[axis]
        };
        triangles.sort_by(|a, b| on_axis(a).partial_cmp(&on_axis(b)).unwrap());

        let (a, b) = triangles.split_at_mut(triangles.len() / 2);

        let next_axis = (axis + 1) % 3;
        let children = (
            BvhNode::build(vertices, a, next_axis),
            BvhNode::build(vertices, b, next_axis),
        );

        BvhNode {
            aabb: children.0.aabb.merged(children.1.aabb),
            content: BvhContent::Node(Box::new(children)),
        }
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