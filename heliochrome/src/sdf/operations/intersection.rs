use crate::{hittables::AABB, maths::vec3, sdf::SDF};

pub struct Intersection<A: SDF, B: SDF> {
    pub a: A,
    pub b: B,
}

impl<A: SDF, B: SDF> SDF for Intersection<A, B> {
    fn dist(&self, p: vec3) -> f32 {
        let d1 = self.a.dist(p);
        let d2 = self.b.dist(p);

        (d1).max(d2)
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        Some(AABB::intersection(
            &self.a.make_bounding_box()?,
            &self.b.make_bounding_box()?,
        ))
    }
}
