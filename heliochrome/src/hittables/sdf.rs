use std::sync::Arc;

use super::{Hit, Hittable, Intersect, AABB};
use crate::{maths::Ray, sdf::SDF};

const MIN_DIST: f32 = 0.000001;
const MAX_MARCHES: u16 = 500;
const MARCH_T_MAX: f32 = 10000.0;

#[derive(Clone)]
pub struct HittableSDF {
    sdf: Arc<dyn SDF>,
}

impl HittableSDF {
    pub fn new(de: impl SDF + 'static) -> Self {
        Self { sdf: Arc::new(de) }
    }
}

impl Hittable for HittableSDF {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let intersection = self.intersect(ray, t_min, t_max)?;

        Some(Hit::new(
            ray,
            intersection.t,
            self.sdf.normal_at(&ray.at(intersection.t)),
        ))
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersect> {
        let mut t = t_min;
        let mut p = ray.at(t);

        for _ in 0..MAX_MARCHES {
            let d = self.sdf.dist(p);
            t += d;
            if t > t_max || t > MARCH_T_MAX || t < t_min {
                break;
            }
            p = ray.at(t);
            if d.abs() < MIN_DIST {
                return Some(Intersect { t, i: 0 });
            }
        }

        None
    }

    fn make_bounding_box(&self) -> AABB {
        self.sdf.make_bounding_box()
    }
}
