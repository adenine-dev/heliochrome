use std::error::Error;

use super::{BounceInfo, Hittable, Intersection};
use crate::{
    loader::{parse_into, FromHCY},
    maths::{vec3, Ray},
};

#[derive(Debug, Clone, Copy, Default)]
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

    pub fn intersection(a: &AABB, b: &AABB) -> Self {
        Self::new(a.min.max(&b.min), a.max.min(&b.max))
    }

    pub fn surface_area(&self) -> f32 {
        let size = self.max - self.min;
        2.0 * (size.x) * (size.y) + (size.x) * (size.z) + (size.y) * (size.z)
    }

    #[inline]
    pub fn hits(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = ray.direction.recip();
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;
        let t_smaller = t0.min(&t1);
        let t_bigger = t0.max(&t1);
        let t_min = t_min.max(t_smaller.max_component());
        let t_max = t_max.min(t_bigger.min_component());

        t_min < t_max
    }
}

impl Hittable for AABB {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let mut entrance = t_min - f32::EPSILON;
        let mut exit = t_max;
        let mut i = 0;

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
                i = ray.direction[a].signum().max(0.0) as u32 + a as u32 * 2;
            }
        }

        if entrance < t_min {
            return None;
        }

        Some(Intersection { t: entrance, i })
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        let normals = [
            vec3::unit_x(),
            -vec3::unit_x(),
            vec3::unit_y(),
            -vec3::unit_y(),
            vec3::unit_z(),
            -vec3::unit_z(),
        ];
        BounceInfo::new(ray, intersection.t, normals[intersection.i as usize])
    }

    fn make_bounding_box(&self) -> AABB {
        *self
    }
}

impl FromHCY for AABB {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut min = None;
        let mut max = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "min" => min = Some(parse_into(value)?),
                "max" => max = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(AABB::new(
            min.ok_or("missing required key `min`")?,
            max.ok_or("missing required key `max`")?,
        ))
    }
}
