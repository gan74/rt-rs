use std::path::Path;

use crate::vec::*;
use crate::vertex::*;
use crate::mesh::*;
use crate::transform::*;
use crate::ray::*;
use crate::hit::*;
use crate::camera::*;
use crate::color::*;
use crate::material::*;

use gltf;

pub struct Scene {
    meshes: Vec<Mesh>,
    camera: Camera,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            meshes: Vec::new(),
            camera: Camera::new(Transform::identity(), 60.0_f32.to_radians(), 1.0),
        }
    }

    pub fn camera(&self) -> Camera {
        self.camera
    }
}

impl Hittable for Scene {
    type Result = HitRecord;

    fn hit(&self, mut ray: Ray) -> Option<Self::Result> {
        let mut hit_rec: Option<HitRecord> = None;
        for mesh in self.meshes.iter() {
            if let Some(hit) = mesh.hit(ray) {
                ray = ray.with_max(hit.dist);
                hit_rec = Some(hit);
            }
        }
        hit_rec
    }
}




pub fn import_scene<P: AsRef<Path>>(path: P) -> gltf::Result<Scene> {
    let (document, buffers, _images) = gltf::import(path)?;


    let mut scene = Scene::new();

    let mut nodes = document.scenes().flat_map(|s| s.nodes()).map(|n| (Transform::identity(), n)).collect::<Vec<_>>();
    while !nodes.is_empty() {
        let mut children = Vec::new();

        for (parent, node) in nodes {
            let transform = parent.then(import_transform(node.transform()));
            children.extend(node.children().map(|n| (transform, n)));

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
                            let material = import_material(primitive.material());
                            scene.meshes.push(Mesh::new_with_material(vertices, triangles, material));
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
                        scene.camera = Camera::new(transform, p.yfov(), p.aspect_ratio().unwrap_or(1.0));
                    },

                    _ => {

                    }
                }
            }
        }
        nodes = children;
    }


    {
        println!("camera.position = {}", scene.camera.position());
        println!("camera.forward  = {}", scene.camera.forward());
        println!("camera.right    = {}", scene.camera.right());
        println!("camera.up       = {}", scene.camera.up());
        println!("{} meshes", scene.meshes.len());
    }

    Ok(scene)
}

fn import_material(mat: gltf::Material) -> Material {
    let pbr = mat.pbr_metallic_roughness();
    let to_color = |col: &[f32]| Color::new(col[0], col[1], col[2]);

    let color = to_color(&pbr.base_color_factor()[0..3]);

    if pbr.metallic_factor() > 0.5 {
        Material::Metal {
            color: color,
            fuzz: pbr.roughness_factor(),
        }
    } else {
        Material::Diffuse(color)
    }
}

fn import_transform(tr: gltf::scene::Transform) -> Transform {
    let matrix = tr.clone().matrix();
    let column = |col: usize| Vec3::new(matrix[col][0], matrix[col][1], matrix[col][2]);
    Transform::from_basis(column(0), column(1), column(2)).with_pos(column(3))
}