use std::fmt::Debug;

use crate::{
    hittables::{Hittable, Intersection},
    maths::Ray,
};

pub trait Accelerator<T: Hittable>: Clone + Debug {
    fn new(objs: Vec<T>) -> Self;

    fn intersect_with_index(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<(Intersection, usize)>;

    fn get_nth(&self, idx: usize) -> &T;

    fn intersect_obj(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<(Intersection, &T)> {
        let (intersection, idx) = self.intersect_with_index(ray, t_min, t_max)?;
        Some((intersection, self.get_nth(idx)))
    }
}

mod bvh;
mod unaccel;

pub type Accel<T> = bvh::BVH<T>;
