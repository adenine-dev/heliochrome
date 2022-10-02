use crate::{
    maths::{vec2, vec3},
    sdf::SDF,
};

pub struct Torus {
    p: vec3,
    r_major: f32,
    r_minor: f32,
}

impl Torus {
    pub fn new(p: vec3, r_major: f32, r_minor: f32) -> Self {
        Torus {
            p,
            r_major,
            r_minor,
        }
    }
}

impl SDF for Torus {
    fn dist(&self, p: vec3) -> f32 {
        let q = vec2::new((p.x * p.x + p.z * p.z).sqrt() - self.r_major, p.y);
        q.mag() - self.r_minor
    }
}
