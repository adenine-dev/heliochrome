use crate::maths::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> f32;
}
