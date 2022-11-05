// This structure is the equivalent to no acceleration structure being used
use super::Accelerator;
use crate::{
    hittables::{Hittable, Intersection},
    maths::Ray,
};

#[derive(Clone)]
pub struct Unaccel<T: Hittable> {
    pub hittables: Vec<T>,
}

impl<T: Hittable> Accelerator<T> for Unaccel<T> {
    fn new(hittables: Vec<T>) -> Self {
        Self { hittables }
    }

    fn intersect_with_index(
        &self,
        ray: &Ray,
        t_min: f32,
        mut t_max: f32,
    ) -> Option<(Intersection, usize)> {
        let mut ret = None;
        for (idx, h) in self.hittables.iter().enumerate() {
            if let Some(i) = h.intersect(ray, t_min, t_max) {
                if i.t <= t_max {
                    t_max = i.t;
                    ret = Some((i, idx))
                }
            }
        }

        ret
    }

    fn get_nth(&self, idx: usize) -> &T {
        &self.hittables[idx]
    }
}
