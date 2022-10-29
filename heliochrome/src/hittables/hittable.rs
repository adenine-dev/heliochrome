use enum_dispatch::enum_dispatch;

use super::AABB;
use crate::{maths::vec3, maths::Ray};

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
#[allow(unused_variables)] // default trait impls
pub trait Hittable: Send + Sync + Clone {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    fn make_bounding_box(&self) -> AABB;

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        panic!("oof you need to implement this :<");
    }

    fn random(&self, origin: &vec3) -> vec3 {
        panic!("haha you need to implement this :<");
    }
}
