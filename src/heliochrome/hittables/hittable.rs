use enum_dispatch::enum_dispatch;

use crate::{heliochrome::maths::vec3, maths::Ray};

use super::AABB;

pub struct Hit {
    pub t: f32,
    pub p: vec3,
    pub normal: vec3,
    pub front_face: bool,
}

impl Hit {
    pub fn new(src_ray: &Ray, t: f32, normal: vec3) -> Self {
        let front_face = src_ray.direction.dot(normal) < 0.0;

        Self {
            t,
            p: src_ray.at(t),
            normal: if front_face { normal } else { -normal },
            front_face,
        }
    }

    pub fn set_normal(&mut self, src_ray: &Ray, normal: vec3) {
        self.front_face = src_ray.direction.dot(normal) < 0.0;
        self.normal = if src_ray.direction.dot(normal) < 0.0 {
            normal
        } else {
            -normal
        };
    }
}

#[enum_dispatch]
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    fn make_bounding_box(&self) -> Option<AABB>;
}
