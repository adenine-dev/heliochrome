use crate::{heliochrome::maths::vec3, maths::Ray};

pub struct Hit {
    pub t: f32,
    pub normal: vec3,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}
