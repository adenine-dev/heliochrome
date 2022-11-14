use crate::{hittables::AABB, maths::vec3, sdf::SDF};

#[derive(Debug)]
pub struct AAB {
    pub extent: vec3,
}

impl AAB {
    pub fn new(extent: vec3) -> Self {
        Self { extent }
    }
}

impl SDF for AAB {
    fn dist(&self, p: vec3) -> f32 {
        let q = p.abs() - self.extent;
        q.max(&vec3::splat(0.0)).mag() + q.x.max(q.y).max(q.z).min(0.0)
    }

    fn make_bounding_box(&self) -> AABB {
        AABB::new(-self.extent, self.extent)
    }
}
