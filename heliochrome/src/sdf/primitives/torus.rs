use crate::{
    hittables::AABB,
    maths::{vec2, vec3},
    sdf::SDF,
};

pub struct Torus {
    r_major: f32,
    r_minor: f32,
}

impl Torus {
    pub fn new(r_major: f32, r_minor: f32) -> Self {
        Torus { r_major, r_minor }
    }
}

impl SDF for Torus {
    fn dist(&self, p: vec3) -> f32 {
        let q = vec2::new((p.x * p.x + p.z * p.z).sqrt() - self.r_major, p.y);
        q.mag() - self.r_minor
    }

    fn make_bounding_box(&self) -> AABB {
        let extent = vec3::new(
            self.r_major + self.r_minor,
            self.r_minor,
            self.r_major + self.r_minor,
        );
        AABB::new(-extent, extent)
    }
}
