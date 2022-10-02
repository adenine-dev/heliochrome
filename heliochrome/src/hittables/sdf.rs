use std::sync::Arc;

use super::{Hit, Hittable, AABB};
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
    fn hit(&self, ray: &Ray, mut t_min: f32, t_max: f32) -> Option<Hit> {
        let mut p = ray.at(t_min);

        for _ in 0..MAX_MARCHES {
            let d = self.sdf.dist(p);
            t_min += d;
            if t_min > t_max || t_min > MARCH_T_MAX {
                break;
            }
            p = ray.at(t_min);
            if d < MIN_DIST {
                return Some(Hit::new(ray, t_min, self.sdf.normal_at(&p)));
            }
        }

        None
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        self.sdf.make_bounding_box()
    }
}
