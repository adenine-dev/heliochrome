use crate::heliochrome::maths::{vec3, Ray};

use super::{Hit, Hittable};

pub struct InfinitePlane {
    pub origin: vec3,
    pub normal: vec3,
}

impl InfinitePlane {
    pub fn new(origin: vec3, normal: vec3) -> Self {
        InfinitePlane { origin, normal }
    }
}

impl Hittable for InfinitePlane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let d = self.normal.dot(ray.direction);
        if d.abs() >= 0.0001 {
            let t = (self.origin - ray.origin).dot(self.normal) / d;
            if t_min <= t && t <= t_max {
                return Some(Hit::new(ray, t, self.normal));
            }
        }

        None
    }
}
