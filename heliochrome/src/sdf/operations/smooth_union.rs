use crate::{
    hittables::AABB,
    maths::{lerp, vec3},
    sdf::SDF,
};

#[derive(Debug)]
pub struct SmoothUnion<A: SDF, B: SDF> {
    pub k: f32,
    pub a: A,
    pub b: B,
}

impl<A: SDF, B: SDF> SDF for SmoothUnion<A, B> {
    fn dist(&self, p: vec3) -> f32 {
        let d1 = self.a.dist(p);
        let d2 = self.b.dist(p);
        let h = (0.5 + 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        lerp(d2, d1, h) - self.k * h * (1.0 - h)

        // let h = (self.k - (d1 - d2).abs()).max(0.0) / self.k;
        // d1.min(d2) - h * h * self.k * (1.0 / 4.0)
    }

    fn make_bounding_box(&self) -> AABB {
        let a = self.a.make_bounding_box();
        let b = self.b.make_bounding_box();
        AABB::new(
            a.min.min(&b.min) - vec3::splat(self.k),
            a.max.max(&b.max) + vec3::splat(self.k),
        )
    }
}
