use super::*;
use crate::heliochrome::maths;

pub struct HittableList {
    pub hittables: Vec<HittableObject>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &maths::Ray, t_min: f32, mut t_max: f32) -> Option<Hit> {
        let mut hit: Option<Hit> = None;

        for hittable in self.hittables.iter() {
            let new_hit = hittable.hit(ray, t_min, t_max);
            if new_hit.is_some() {
                hit = new_hit;
                t_max = hit.as_ref().unwrap().t;
            }
        }

        hit
    }
}
