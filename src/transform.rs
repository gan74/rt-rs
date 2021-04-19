
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

    pub fn forward(&self) -> Vec3 {
        self.basis[1]
    }

    pub fn right(&self) -> Vec3 {
        self.basis[0]
    }

    pub fn up(&self) -> Vec3 {
        self.basis[2]
    }

    pub fn position(&self) -> Vec3 {
        self.pos
    }
}