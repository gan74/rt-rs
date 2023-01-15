
use crate::vec::*;
use crate::ray::*;
use crate::hit::*;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    min: Vec3,
    max: Vec3
}

pub trait WithAabb {
    fn aabb(&self) -> Aabb;
}


impl Aabb {
    pub fn empty(center: Vec3) -> Aabb {
        Aabb {
            min: center,
            max: center
        }
    }

    pub fn merged(self, other: Aabb) -> Aabb {
        Aabb {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn from_points<T: Iterator<Item = Vec3>>(points: T) -> Option<Aabb> {
        points.map(|p| Aabb::empty(p)).reduce(|a, b| Aabb::merged(a, b))
    }

    pub fn contains(&self, pos: Vec3) -> bool {
        pos.x > self.min.x &&
        pos.y > self.min.y &&
        pos.z > self.min.z &&

        pos.x < self.max.x &&
        pos.y < self.max.y &&
        pos.z < self.max.z
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }
}


impl Hittable for Aabb {
    type Result = ();

    fn hit(&self, ray: Ray) -> Option<Self::Result> {
        let mut near: f32 = 0.0;
        let mut far = ray.max;
        for i in 0..3 {
            let origin = ray.orig[i];
            let direction = ray.dir[i];
            let min = self.min[i];
            let max = self.max[i];

            if direction == 0.0 {
                if origin < min || origin > max {
                    return None;
                }
            } else {
                let inv_dir = 1.0 / direction;
                let t1: f32 = (min - origin) * inv_dir;
                let t2: f32 = (max - origin) * inv_dir;

                let (t1, t2) = (t1.min(t2), t1.max(t2));

                near = near.max(t1);
                far = far.min(t2);

                if far < near {
                    return None;
                }
            }
        }

        Some(())
    }
}
