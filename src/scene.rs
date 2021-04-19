use std::path::Path;

use crate::vec::*;
use crate::mesh::*;
use crate::transform::*;
use crate::ray::*;
use crate::hit::*;
use crate::camera::*;
use crate::color::*;

use gltf;



pub struct PointLight {
    shape: Sphere,
    intensity: f32,
}

pub struct Scene {
    meshes: Vec<Mesh>,
    camera: Camera,

    point_lights: Vec<PointLight>,
}




impl Scene {
    pub fn new() -> Scene {
        Scene {
            meshes: Vec::new(),
            camera: Camera::new(Transform::identity(), 60.0_f32.to_radians(), 1.0),
            point_lights: Vec::new(),
        }
    }

    pub fn camera(&self) -> Camera {
        self.camera
    }


    pub fn trace(&self, u: f32, v: f32) -> Color {
        let ray = self.camera.generate_ray(u, v);

        match self.hit(&ray) {
            Some(hit) => {
                Color {
                    r: hit.norm.x * 0.5 + 0.5,
                    g: hit.norm.y * 0.5 + 0.5,
                    b: hit.norm.z * 0.5 + 0.5,
                }
            },

            None => Color::from(0.0)
        }
    }
}

impl Hittable for Scene {
    type Result = HitRecord;

    fn hit(&self, ray: &Ray) -> Option<Self::Result> {
        let mut hit_rec: Option<HitRecord> = None;

        for mesh in self.meshes.iter() {
            if let Some(hit) = mesh.hit(&ray) {
                if hit_rec.is_none() || hit.dist < hit_rec.unwrap().dist {
                    hit_rec = Some(hit);
                }
            }
        }

        hit_rec
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
                    match (reader.read_positions(), reader.read_normals(), reader.read_indices()) {
                        (Some(positions), Some(normals), Some(indices)) => {
                            let vertices = positions.zip(normals).map(|(p, n)| Vertex {
                                pos: transform.transform_pos(Vec3::from(p)),
                                norm: transform.transform_dir(Vec3::from(n)).normalized(),
                            }).collect();
                            let indices = indices.into_u32().collect::<Vec<_>>();
                            let triangles = indices.as_slice().chunks(3).map(|sl| [sl[0], sl[1], sl[2]]).collect();
                            scene.meshes.push(Mesh::new(vertices, triangles));
                        },

                        _ => {
                            eprintln!("Incomplete mesh");
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

    {
        let light = PointLight {
            shape: Sphere {
                center: Vec3::new(5.0, 1.0, 6.0),
                radius: 0.25,
            },
            intensity: 10.0
        };
        scene.point_lights.push(light);
    }

    Ok(scene)
}


fn import_transform(tr: gltf::scene::Transform) -> Transform {
    let matrix = tr.clone().matrix();
    let column = |row: usize| Vec3::new(matrix[row][0], matrix[row][1], matrix[row][2]);
    Transform::from_basis(column(0), column(1), column(2)).with_pos(column(3))
}