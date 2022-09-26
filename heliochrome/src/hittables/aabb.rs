use rayon::vec;

use crate::maths::{vec3, Ray};

use super::{Hit, Hittable};

#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub min: vec3,
    pub max: vec3,
}

impl AABB {
    pub fn new(min: vec3, max: vec3) -> Self {
        Self { min, max }
    }

    pub fn surrounding(a: &AABB, b: &AABB) -> Self {
        Self::new(a.min.min(&b.min), a.max.max(&b.max))
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.max - self.min;
        2.0 * (size.x) * (size.y) + (size.x) * (size.z) + (size.y) * (size.z)
    }

    pub fn hits(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
            }
            t_max = t_max.min(t1);
            t_min = t_min.max(t0);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> Option<Hit> {
        let normals = [vec3::unit_x(), vec3::unit_y(), vec3::unit_z()];
        let mut entrance = t_min - f32::EPSILON;
        let mut exit = t_max;
        let mut n = vec3::splat(0.0);

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut close = (self.min[a] - ray.origin[a]) * inv_d;
            let mut far = (self.max[a] - ray.origin[a]) * inv_d;
            if far < close {
                (close, far) = (far, close);
            }

            if far < entrance || close > exit {
                return None;
            }

            exit = exit.min(far);
            if close > entrance {
                entrance = close;
                n = ray.direction[a].signum() * normals[a];
            }
        }

        if entrance < t_min {
            return None;
        }

        Some(Hit::new(ray, entrance, n))
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        return Some(*self);
    }
}
