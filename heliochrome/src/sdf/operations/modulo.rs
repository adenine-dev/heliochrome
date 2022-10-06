//TODO: this has some precision issues..

use crate::{hittables::AABB, maths::vec3, sdf::SDF};

pub struct Modulo<A: SDF> {
    pub primitive: A,
    pub period: vec3,
}

impl<A: SDF> SDF for Modulo<A> {
    fn dist(&self, p: vec3) -> f32 {
        self.primitive.dist(
            (p + 0.5 * self.period)
                - self.period * ((p + 0.5 * self.period) / self.period).floor()
                - 0.5 * self.period,
        )
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        None
    }
}
