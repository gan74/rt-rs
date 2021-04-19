
use crate::vec::*;
use crate::ray::*;

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

    pub fn contains(&self, pos: Vec3) -> bool {
        pos.x > self.min.x &&
        pos.y > self.min.y &&
        pos.z > self.min.z &&

        pos.x < self.max.x &&
        pos.y < self.max.y &&
        pos.z < self.max.z
    }
}


impl Intersectable for AABB {
    type Result = ();

    fn intersects(&self, ray: &Ray) -> Option<Self::Result> {
        let origin: [f32; 3] = ray.origin().into();
        let direction: [f32; 3] = ray.direction().into();
        let min: [f32; 3] = self.min.into();
        let max: [f32; 3] = self.max.into();

        let mut near = -f32::MAX;
        let mut far = f32::MAX;
        for i in 0..3 {
            let origin = origin[i];
            let min = min[i];
            let max = max[i];

            if direction[i] == 0.0 {
                if origin < min || origin > max {
                    return None;
                }
            } else {
                let inv_dir = 1.0 / direction[i];
                let t1 = (min - origin) * inv_dir;
                let t2 = (max - origin) * inv_dir;

                let (t1, t2) = (t1.min(t2), t1.max(t2));

                near = near.max(t1);
                far = far.min(t2);

                if far < near {
                    return None;
                }
            }
        }

        if far >= 0.0 {
            Some(())
        } else {
            None
        }
    }
}
