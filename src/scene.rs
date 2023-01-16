use std::path::Path;

use crate::vec::*;
use crate::aabb::*;
use crate::vertex::*;
use crate::mesh::*;
use crate::bvh::*;
use crate::transform::*;
use crate::ray::*;
use crate::hit::*;
use crate::camera::*;
use crate::color::*;
use crate::surface::*;
use crate::material::*;

use rand::prelude::*;

use gltf;


const MAX_OBJECT_PER_NODE: usize = 2;


pub type SceneObject = Mesh;


pub struct Scene {
    objects: Vec<SceneObject>,
    bvh: Bvh<u32>,

    emitters: Vec<u32>,
    emitter_area: f32,

    camera: Camera,
}

pub struct SceneBuilder {
    objects: Vec<SceneObject>,
    camera: Camera,
}


impl Scene {
    pub fn camera(&self) -> Camera {
        self.camera
    }

    pub fn sample_emitter_surface<R: RngCore>(&self, rng: &mut R) -> Option<(&SceneObject, Color)> {
        if self.emitters.is_empty() {
            return None;
        }

        let index = self.emitters[rng.gen_range(0..self.emitters.len())] as usize;
        let emitter = &self.objects[index];
        let norm = emitter.area() / self.emitter_area;
        return Some((emitter, emitter.material().emissive * norm));
    } 


    fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            bvh: Bvh::empty(),

            emitters: Vec::new(),
            emitter_area: 0.0,

            camera: Camera::new(Transform::identity(), 60.0_f32.to_radians(), 1.0),
        }
    }

    fn build_bvh(&mut self) {
        let mut indices = (0..self.objects.len() as u32).collect::<Vec<_>>();
        let object_aabb = |i: &u32| self.objects[*i as usize].aabb();
        self.bvh = Bvh::new(indices.as_mut_slice(), object_aabb, MAX_OBJECT_PER_NODE);
    }

    fn build_emitters(&mut self) {
        self.emitters = self.objects.iter().enumerate().filter(|obj| obj.1.material().is_emissive()).map(|i| i.0 as u32).collect();
        self.emitter_area = self.emitters.iter().fold(0.0, |area, i| area + self.objects[*i as usize].area());
    }
}


impl SceneBuilder {
    pub fn new() -> SceneBuilder {
        SceneBuilder {
            objects: Vec::new(),
            camera: Camera::new(Transform::identity(), 60.0_f32.to_radians(), 1.0),
        }
    }

    pub fn push(&mut self, obj: SceneObject) {
        self.objects.push(obj);
    } 

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    } 

    pub fn build(self) -> Scene {
        let mut scene = Scene::new();

        scene.objects = self.objects;
        scene.camera = self.camera;

        scene.build_bvh();
        scene.build_emitters();

        scene
    }
}


impl<'scene> Hittable for &'scene Scene {
    type Result = HitRecord<'scene>;

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        self.bvh.trace(ray, |mut r, objects| {
            let mut hit_rec: Option<Self::Result> = None;
            for i in objects {
                let obj = &self.objects[*i as usize];
                if let Some(hit) = obj.hit(r) {
                    r = r.with_max(hit.dist);
                    hit_rec = Some(hit);
                }
            }
            hit_rec
        })
    }
}



pub fn import_scene<P: AsRef<Path>>(path: P) -> gltf::Result<Scene> {
    let (document, buffers, _images) = gltf::import(path)?;


    let mut builder = SceneBuilder::new();

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
                            builder.push(Mesh::new(vertices, triangles, material));
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
                        builder.set_camera(Camera::new(transform, p.yfov(), p.aspect_ratio().unwrap_or(1.0)));
                    },

                    _ => {

                    }
                }
            }
        }
        nodes = children;
    }

    let scene = builder.build();

    {
        println!("camera.position = {}", scene.camera.position());
        println!("camera.forward  = {}", scene.camera.forward());
        println!("camera.right    = {}", scene.camera.right());
        println!("camera.up       = {}", scene.camera.up());
        println!("{} emitters", scene.emitters.len());
        println!("{} objects", scene.objects.len());
    }

    Ok(scene)
}

fn import_material(mat: gltf::Material) -> Material {
    let to_color = |col: &[f32]| Color::new(col[0], col[1], col[2]);

    let pbr = mat.pbr_metallic_roughness();
    let kind = if pbr.metallic_factor() > 0.5 {
        MaterialKind::Metal{fuzz: pbr.roughness_factor()}
    } else {
        MaterialKind::Diffuse
    };

    Material {
        kind: kind,
        color: to_color(&pbr.base_color_factor()[0..3]),
        emissive: to_color(&mat.emissive_factor()[0..3]),
    }
}

fn import_transform(tr: gltf::scene::Transform) -> Transform {
    let matrix = tr.clone().matrix();
    let column = |col: usize| Vec3::new(matrix[col][0], matrix[col][1], matrix[col][2]);
    Transform::from_basis(column(0), column(1), column(2)).with_pos(column(3))
}