
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    min: Vec3,
    max: Vec3
}

impl AABB {
    pub fn empty(center: Vec3) -> AABB {
        AABB {
            min: center,
            max: center
        }
    }

    pub fn merged(self, other: AABB) -> AABB {
        AABB {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn from_points<T: Iterator<Item = Vec3>>(points: T) -> Option<AABB> {
        points.map(|p| AABB::empty(p)).reduce(|a, b| AABB::merged(a, b))
    }

    pub fn is_inside(&self, pos: Vec3) -> bool {
        pos.x > self.min.x &&
        pos.y > self.min.y &&
        pos.z > self.min.z &&

        pos.x < self.max.x &&
        pos.y < self.max.y &&
        pos.z < self.max.z
    }
}