use crate::vec::*;


pub struct Frame {
    n: Vec3,
    s: Vec3,
    t: Vec3
}

pub struct FrameValues {
    cos_theta: f32,
    sin_theta: f32,
    tan_theta: f32,

    cos_theta_2: f32,
    sin_theta_2: f32,
}


impl Frame {
    pub fn from_normal(norm: Vec3) -> Frame {
        let b = if norm.x.abs() > norm.y.abs() {
            let inv_len = 1.0 / (norm.y * norm.y + norm.z * norm.z).sqrt();
            Vec3::new(0.0, norm.z * inv_len, -norm.y * inv_len)
        } else {
            let inv_len = 1.0 / (norm.x * norm.x + norm.z * norm.z).sqrt();
            Vec3::new(norm.z * inv_len, 0.0, -norm.x * inv_len)
        };

        Frame {
            n: norm,
            s: b.cross(norm),
            t: b
        }
    }

    pub fn to_local(&self, v: Vec3) -> Vec3 {
        Vec3::new(v.dot(self.s), v.dot(self.t), v.dot(self.n))
    }

    pub fn to_world(&self, v: Vec3) -> Vec3 {
        self.s * v.x + self.t * v.y + self.n * v.z
    }

    pub fn values(&self, v: Vec3) -> FrameValues {
        let cos_theta_2 = v.z * v.z;
        let sin_theta_2 = 1.0 - cos_theta_2;
        let sin_theta = sin_theta_2.sqrt();
        let tan_theta = sin_theta / v.z;

        FrameValues {
            cos_theta: v.z,
            sin_theta: sin_theta,
            tan_theta: tan_theta,

            cos_theta_2: cos_theta_2,
            sin_theta_2: sin_theta_2,
        }
    }
} 