
use crate::aabb::*;
use crate::ray::*;
use crate::vec::*;
use crate::hit::*;

pub struct Bvh<T> {
    root: BvhNode<T>,
}

struct BvhNode<T> {
    aabb: Aabb,
    content: BvhContent<T>,
}

enum BvhContent<T> {
    Leaf(Vec<T>),
    Node(Box<(BvhNode<T>, BvhNode<T>)>),
}


impl<T: Clone> Bvh<T> {
    pub fn new<F: Fn(&T) -> Aabb>(objects: &mut [T], to_aabb: F, max_object_per_node: usize) -> Bvh<T> {
        debug_assert!(!objects.is_empty());
        Bvh {
            root: BvhNode::build(objects, &to_aabb, max_object_per_node, 0)
        }
    }

    pub fn empty() -> Bvh<T> {
        Bvh {
            root: BvhNode {
                aabb: Aabb::empty(Vec3::zero()),
                content: BvhContent::Leaf(Vec::new())
            },
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.root.content {
            BvhContent::Leaf(objects) => objects.is_empty(),
            _ => false,
        }
    }

    pub fn aabb(&self) -> Aabb {
        self.root.aabb
    }

    pub fn trace<F: Fn(Ray, &[T]) -> Option<HitRecord>>(&self, ray: Ray, hit_func: F) -> Option<HitRecord> {
        Self::trace_node(&self.root, ray, &hit_func)
    }



    fn trace_node<F: Fn(Ray, &[T]) -> Option<HitRecord>>(node: &BvhNode<T>, mut ray: Ray, hit_func: &F) -> Option<HitRecord> {
        if node.aabb.hit(ray).is_none() {
            return None;
        }
        match &node.content {
            BvhContent::Leaf(objects) => hit_func(ray, objects),
            BvhContent::Node(children) => {
                let dist_sq = |c: &BvhNode<T>| c.aabb.center().distance2(ray.orig);

                let children = if dist_sq(&children.0) < dist_sq(&children.1) {
                    [&children.0, &children.1]
                } else {
                    [&children.1, &children.0]
                };

                let mut hit_rec: Option<HitRecord> = None;
                for child in children.iter() {
                    if let Some(hit) = Bvh::trace_node(child, ray, hit_func) {
                        ray = ray.with_max(hit.dist);
                        hit_rec = Some(hit);
                    }
                }
                hit_rec
            },
        }
    }
}


impl<T: Clone> BvhNode<T> {
    fn build<F: Fn(&T) -> Aabb>(objects: &mut [T], to_aabb: &F, max_object_per_node: usize, axis: usize) -> BvhNode<T> {
        debug_assert!(!objects.is_empty());

        if objects.len() <= max_object_per_node {
            return BvhNode {
                aabb: objects.iter().map(|o| to_aabb(o)).reduce(|acc, e| acc.merged(e)).unwrap(),
                content: BvhContent::Leaf(Vec::from(objects)),
            };
        }

        let on_axis = |obj: &T| {
            to_aabb(obj).center()[axis]
        };
        objects.sort_by(|a, b| on_axis(a).partial_cmp(&on_axis(b)).unwrap());

        let (a, b) = objects.split_at_mut(objects.len() / 2);
        debug_assert!(!a.is_empty());
        debug_assert!(!b.is_empty());

        let next_axis = (axis + 1) % 3;
        let children = (
            Self::build(a, to_aabb, max_object_per_node, next_axis),
            Self::build(b, to_aabb, max_object_per_node, next_axis),
        );

        BvhNode {
            aabb: children.0.aabb.merged(children.1.aabb),
            content: BvhContent::Node(Box::new(children)),
        }
    }
}