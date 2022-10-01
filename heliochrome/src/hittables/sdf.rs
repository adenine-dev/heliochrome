use std::sync::Arc;

use super::{Hit, Hittable, AABB};
use crate::maths::{vec3, Ray};

const NORMAL_H: f32 = 0.00001;

pub trait DistEstimator: Send + Sync {
    fn dist(&self, p: vec3) -> f32;

    fn normal_at(&self, p: &vec3) -> vec3 {
        (vec3::new(
            self.dist(p + vec3::unit_x() * NORMAL_H) - self.dist(p - vec3::unit_x() * NORMAL_H),
            self.dist(p + vec3::unit_y() * NORMAL_H) - self.dist(p - vec3::unit_y() * NORMAL_H),
            self.dist(p + vec3::unit_z() * NORMAL_H) - self.dist(p - vec3::unit_z() * NORMAL_H),
        ))
        .normalized()
    }
}

#[derive(Clone)]
pub struct SDF {
    dist_estimator: Arc<dyn DistEstimator>,
}

impl SDF {
    pub fn new(de: impl DistEstimator + 'static) -> Self {
        Self {
            dist_estimator: Arc::new(de),
        }
    }
}

impl Hittable for SDF {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let mut t = t_min;
        let mut p = ray.at(t);
        let max_iterations = 100;
        let min_dist = 0.001;
        for _ in 0..max_iterations {
            let d = self.dist_estimator.dist(p);
            t += d;
            if t > t_max {
                break;
            }
            p = ray.at(t);
            if d < min_dist {
                return Some(Hit::new(ray, t, self.dist_estimator.normal_at(&p)));
            }
        }

        None
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        None
    }
}

pub struct SphereSDF {
    pub r: f32,
    pub c: vec3,
}

impl DistEstimator for SphereSDF {
    fn dist(&self, p: vec3) -> f32 {
        (p - self.c).mag() - self.r
    }
}
