use std::path::Path;

use crate::vec::*;
use crate::mesh::*;
use crate::transform::*;
use crate::ray::*;

use gltf;


#[derive(Debug, Clone, Copy)]
pub struct Camera {
    transform: Transform,
    tan_half_vfov: f32,
    ratio: f32,
}

pub struct Scene {
    meshes: Vec<(Transform, Mesh)>,
    camera: Camera,
}




impl Camera {
    pub fn new(tr: Transform, vfov: f32, ratio: f32) -> Camera {
        Camera {
            transform: tr,
            tan_half_vfov: (vfov * 0.5).tan(),
            ratio: ratio,
        }
    }

    pub fn generate_ray(&self, u: f32, v: f32) -> Ray {
        let x = (u * 2.0 - 1.0) * self.tan_half_vfov;
        let y = (v * 2.0 - 1.0) * self.tan_half_vfov;

        let dir = self.transform.right() * x + self.transform.up() * y + self.transform.forward();
        Ray::new(self.transform.position(), dir)
    }

    pub fn transform(&self) -> Transform {
        self.transform
    }

    pub fn ratio(&self) -> f32 {
        self.ratio
    }
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            meshes: Vec::new(),
            camera: Camera::new(Transform::identity(), 60.0_f32.to_radians(), 1.0),
        }
    }

    pub fn trace_ray(&self, u: f32, v: f32) -> Option<Vec3> {
        let ray = self.camera.generate_ray(u, v);

        let mut hit_pos = None;
        let mut depth_sq = f32::MAX;

        for (transform, mesh) in self.meshes.iter() {
            for tri in mesh.triangles() {
                let hit = intersect(&ray, tri);
                if let Some(bary) = hit {
                    let pos = tri[0] * bary.0 + tri[1] * bary.1 + tri[2] * bary.2;
                    let dist_sq = ray.origin().distance2(pos);

                    if dist_sq < depth_sq {
                        depth_sq = dist_sq;
                        hit_pos = Some(pos);
                        //hit_pos = Some(Vec3::new(bary.0, bary.1, bary.2));
                    }
                }
            }
        }

        hit_pos
    }

    pub fn camera(&self) -> Camera {
        self.camera
    }
}










pub fn import_scene<P: AsRef<Path>>(path: P) -> gltf::Result<Scene> {
    let (document, buffers, _images) = gltf::import(path)?;

    let mut scene = Scene::new();

    if let Some(sce) = document.default_scene() {
        for node in sce.nodes() {
            let transform = import_transform(node.transform());
            if let Some(mesh) = node.mesh() {
                for primitive in mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
                    match (reader.read_positions(), reader.read_indices()) {
                        (Some(positions), Some(indices)) => {
                            let vertices = positions.map(|p| Vertex { pos: Vec3::from(p) }).collect();
                            let indices = indices.into_u32().collect::<Vec<_>>();
                            let triangles = indices.as_slice().chunks(3).map(|sl| [sl[0], sl[1], sl[2]]).collect();
                            let mesh = Mesh::new(vertices, triangles);
                            scene.meshes.push((transform, mesh));
                        },

                        _ => {

                        },
                    }
                }
            }

            if let Some(cam) = node.camera() {
                match cam.projection() {
                    gltf::camera::Projection::Perspective(p) => {
                        //println!("{:?}", node.transform().matrix());
                        let transform = Transform::from_basis(transform.right(), -transform.up(), transform.forward()).with_pos(transform.position());
                        println!("camera.position = {}", transform.position());
                        println!("camera.forward  = {}", transform.forward());
                        println!("camera.right    = {}", transform.right());
                        println!("camera.up       = {}", transform.up());
                        scene.camera = Camera::new(transform, p.yfov(), p.aspect_ratio().unwrap_or(1.0));
                    },

                    _ => {

                    }
                }
            }
        }
    }

    Ok(scene)
}


fn import_transform(tr: gltf::scene::Transform) -> Transform {
    let matrix = tr.clone().matrix();
    let column = |row: usize| Vec3::new(matrix[row][0], matrix[row][1], matrix[row][2]);
    Transform::from_basis(column(0), column(1), column(2)).with_pos(column(3))
}