use crate::heliochrome::maths::{vec3, Ray};

use super::{Hit, Hittable};

pub struct AABB {
    min: vec3,
    max: vec3,
}

impl AABB {
    pub fn new(min: vec3, max: vec3) -> Self {
        Self { min, max }
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> Option<Hit> {
        let mut n = vec3::splat(0.0);
        for a in 0..3 {
            let mut flip = false;
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;
            if inv_d < 0.0 {
                (t0, t1) = (t1, t0);
                flip = true;
            }
            t_max = t_max.min(t1);
            if t0 > t_min {
                t_min = t0;
                n = match a {
                    0 => vec3::unit_x(),
                    1 => vec3::unit_y(),
                    2 => vec3::unit_z(),
                    _ => vec3::splat(0.0),
                } * (if flip { -1.0 } else { 1.0 });
            }
            if t_max <= t_min {
                return None;
            }
        }
        Some(Hit::new(ray, t_min, n))
    }
}
