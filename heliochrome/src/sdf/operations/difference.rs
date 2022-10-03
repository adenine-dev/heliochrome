use crate::{hittables::AABB, maths::vec3, sdf::SDF};

pub struct Difference<A: SDF, B: SDF> {
    pub a: A,
    pub b: B,
}

impl<A: SDF, B: SDF> SDF for Difference<A, B> {
    fn dist(&self, p: vec3) -> f32 {
        let d1 = self.a.dist(p);
        let d2 = self.b.dist(p);

        (-d2).max(d1)
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        self.a.make_bounding_box()
    }
}
