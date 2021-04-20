use std::path::Path;

use crate::vec::*;
use crate::mesh::*;
use crate::transform::*;
use crate::ray::*;
use crate::hit::*;
use crate::camera::*;
use crate::color::*;

use gltf;

use rand::prelude::*;



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

    pub fn generate_ray(&self, rng: &mut ThreadRng, x: u32, y: u32, width: u32, height: u32) -> Ray {
        let x = x as f32;
        let y = y as f32;
        let width = width as f32;
        let height = height as f32;

        let (x_min, x_max) = (x / width, (x + 1.0) / width);
        let (y_min, y_max) = (y / height, (y + 1.0) / height);

        let u = rng.gen::<f32>() * (x_max - x_min) + x_min;
        let v = rng.gen::<f32>() * (y_max - y_min) + y_min;

        self.camera.generate_ray(u, 1.0 - v)
    }

    pub fn trace(&self, ray: Ray, rng: &mut ThreadRng, max_rays: usize) -> Color {
        if max_rays == 0 {
            return Color::from(0.0);
        }

        match self.hit(ray) {
            Some(hit) => {
                let new_dir = hit.norm + random_unit_vector(rng);
                let new_ray = Ray::new_with_epsilon(hit.pos, new_dir);
                self.trace(new_ray, rng, max_rays - 1) * 0.5
            },

            None => {
                let v = ray.dir.z.max(0.0);
                Color::new(0.25, 0.35, 0.5) * v + (1.0 - v)
            },
        }
    }
}

impl Hittable for Scene {
    type Result = HitRecord;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        let mut hit_rec: Option<HitRecord> = None;

        for mesh in self.meshes.iter() {
            if let Some(hit) = mesh.hit(ray) {
                if hit_rec.is_none() || hit.dist < hit_rec.unwrap().dist {
                    hit_rec = Some(hit);
                }
            }
        }

        hit_rec
    }
}





fn random_unit_vector(rng: &mut ThreadRng) -> Vec3 {
    loop {
        let v = Vec3::new(
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
            rng.gen::<f32>() * 2.0 - 1.0,
        );

        if v.length2() <= 1.0 {
            return v.normalized();
        }
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