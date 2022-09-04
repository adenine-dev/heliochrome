use enum_dispatch::enum_dispatch;

use crate::{heliochrome::maths::vec3, materials::Material, maths::Ray};

pub struct Hit<'a> {
    pub t: f32,
    pub normal: vec3,
    pub material: &'a Material,
}

#[enum_dispatch]
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
