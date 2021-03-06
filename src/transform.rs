
use crate::vec::*;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    basis: [Vec3; 3],
    pos: Vec3
}

impl Transform {
    pub fn identity() -> Transform {
        Transform {
            basis: [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ],
            pos: Vec3::zero()
        }
    }

    pub fn from_basis(x: Vec3, y: Vec3, z: Vec3) -> Transform {
        Transform {
            basis: [x, y, z],
            pos: Vec3::zero()
        }
    }

    pub fn with_pos(self, pos: Vec3) -> Transform {
        Transform {
            basis: self.basis,
            pos: pos
        }
    }

    pub fn basis(&self) -> &[Vec3; 3] {
        &self.basis
    }

    pub fn position(&self) -> Vec3 {
        self.pos
    }

    pub fn transform_pos(&self, pos: Vec3) -> Vec3 {
        self.transform_dir(pos) + self.pos
    }

    pub fn transform_dir(&self, dir: Vec3) -> Vec3 {
        self.basis[0] * dir.x +
        self.basis[1] * dir.y +
        self.basis[2] * dir.z
    }

    pub fn then(&self, o: Transform) -> Transform {
        Transform {
            basis: [
                self.transform_dir(o.basis[0]),
                self.transform_dir(o.basis[1]),
                self.transform_dir(o.basis[2]),
            ],
            pos: self.transform_pos(o.pos),
        }
    }
}