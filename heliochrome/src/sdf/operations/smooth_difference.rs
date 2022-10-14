use crate::{
    hittables::AABB,
    maths::{lerp, vec3},
    sdf::SDF,
};

pub struct SmoothDifference<A: SDF, B: SDF> {
    pub k: f32,
    pub a: A,
    pub b: B,
}

impl<A: SDF, B: SDF> SDF for SmoothDifference<A, B> {
    fn dist(&self, p: vec3) -> f32 {
        let d1 = self.a.dist(p);
        let d2 = self.b.dist(p);

        let h = (0.5 - 0.5 * (d2 + d1) / self.k).clamp(0.0, 1.0);
        lerp(d1, -d2, h) + self.k * h * (1.0 - h)
    }

    fn make_bounding_box(&self) -> AABB {
        self.a.make_bounding_box()
    }
}
