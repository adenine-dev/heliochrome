use enum_dispatch::enum_dispatch;

use crate::{heliochrome::maths::vec3, materials::Material, maths::Ray};

pub struct Hit<'a> {
    pub t: f32,
    pub normal: vec3,
    pub material: &'a Material,
    pub front_face: bool,
}

impl<'a> Hit<'a> {
    pub fn new(src_ray: &Ray, t: f32, normal: vec3, material: &'a Material) -> Self {
        let front_face = src_ray.direction.dot(normal) < 0.0;

        Self {
            t,
            normal: if front_face { normal } else { -normal },
            material,
            front_face,
        }
    }
}

#[enum_dispatch]
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
