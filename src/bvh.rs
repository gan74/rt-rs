
use crate::aabb::*;


pub struct BvhNode<T> {
    pub aabb: Aabb,
    pub content: BvhContent<T>,
}

pub enum BvhContent<T> {
    Leaf(Vec<T>),
    Node(Box<(BvhNode<T>, BvhNode<T>)>),
}

impl<T: Clone> BvhNode<T> {
    pub fn new<F: Fn(&T) -> Aabb>(mut objects: Vec<T>, to_aabb: &F) -> BvhNode<T> {
        BvhNode::<T>::build(&mut objects, to_aabb, 0)
    }

    fn build<F: Fn(&T) -> Aabb>(objects: &mut [T], to_aabb: &F, axis: usize) -> BvhNode<T> {
        if objects.len() < 8 {
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

        let next_axis = (axis + 1) % 3;
        let children = (
            BvhNode::<T>::build(a, to_aabb, next_axis),
            BvhNode::<T>::build(b, to_aabb, next_axis),
        );

        BvhNode {
            aabb: children.0.aabb.merged(children.1.aabb),
            content: BvhContent::Node(Box::new(children)),
        }
    }
}