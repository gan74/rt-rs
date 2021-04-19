
use crate::vec::*;
use crate::aabb::*;

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

    pub fn triangles(&self) -> impl Iterator<Item = [Vec3; 3]> + '_ {
        let vertices = self.vertices.as_slice();
        self.triangles.iter().map(move |tri| [
            vertices[tri[0] as usize].pos,
            vertices[tri[1] as usize].pos,
            vertices[tri[2] as usize].pos,
        ])
    }

    pub fn aabb(&self) -> AABB {
        self.aabb
    }
}