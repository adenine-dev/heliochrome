use super::{Hit, Hittable, AABB};
use crate::maths::{vec3, Ray};

#[derive(Clone)]
pub struct Rect {
    origin: vec3,
    sx: vec3,
    sy: vec3,
    normal: vec3,
}

impl Rect {
    pub fn new(origin: vec3, sx: vec3, sy: vec3) -> Self {
        Rect {
            origin,
            sx,
            sy,
            normal: sx.cross(sy).normalize(),
        }
    }
}

impl Hittable for Rect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let d = self.normal.dot(ray.direction);
        if d.abs() >= 0.0001 {
            let t = (self.origin - ray.origin).dot(self.normal) / d;
            if t_min <= t && t <= t_max {
                let p = self.origin - ray.at(t);
                let q1m = ((p.dot(self.sx) / self.sx.mag()) * (self.sx / self.sx.mag())).mag();
                let q2m = ((p.dot(self.sy) / self.sy.mag()) * (self.sy / self.sy.mag())).mag();
                if 0.0 <= q1m && q1m <= self.sx.mag() && 0.0 <= q2m && q2m <= self.sy.mag() {
                    return Some(Hit::new(ray, t, self.normal));
                }
            }
        }

        None
    }
    fn make_bounding_box(&self) -> Option<AABB> {
        let min = self
            .origin
            .min(&(self.origin + self.sx))
            .min(&(self.origin + self.sy))
            .min(&(self.origin + self.sx + self.sy))
            - vec3::splat(0.0001);
        let max = self
            .origin
            .max(&(self.origin + self.sx))
            .max(&(self.origin + self.sy))
            .max(&(self.origin + self.sx + self.sy))
            + vec3::splat(0.0001);

        Some(AABB::new(min, max))
    }
}
