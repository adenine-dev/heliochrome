use crate::{hittables::AABB, maths::vec3, sdf::SDF};

#[derive(Debug)]
pub struct Sphere {
    pub r: f32,
    pub c: vec3,
}

impl Sphere {
    pub fn new(r: f32, c: vec3) -> Self {
        Self { r, c }
    }
}

impl SDF for Sphere {
    fn dist(&self, p: vec3) -> f32 {
        (p - self.c).mag() - self.r
    }

    fn make_bounding_box(&self) -> AABB {
        AABB::new(
            self.c - vec3::splat(self.r.abs()),
            self.c + vec3::splat(self.r.abs()),
        )
    }
}
