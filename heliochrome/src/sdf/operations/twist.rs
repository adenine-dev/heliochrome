//NOTE: this has precision issues :<

use crate::{
    hittables::AABB,
    maths::{mat2, vec2, vec3},
    sdf::SDF,
};

pub struct Twist<P: SDF> {
    pub k: f32,
    pub primitive: P,
}

impl<P: SDF> SDF for Twist<P> {
    fn dist(&self, p: vec3) -> f32 {
        let m = mat2::rotate(self.k * p.y);
        let mpxz = m * vec2::new(p.x, p.z);
        let q = vec3::new(mpxz.x, mpxz.y, p.y);
        self.primitive.dist(q)
    }

    fn make_bounding_box(&self) -> AABB {
        //FIXME: this is def not quite right
        AABB::new(vec3::splat(-f32::INFINITY), vec3::splat(f32::INFINITY))
    }
}
