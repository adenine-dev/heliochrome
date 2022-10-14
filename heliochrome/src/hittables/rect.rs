use super::{Hit, Hittable, AABB};
use crate::maths::{vec3, Ray};

#[derive(Clone)]
pub struct Rect {
    origin: vec3,
    s1: vec3,
    s2: vec3,
    normal: vec3,
}

impl Rect {
    pub fn new(origin: vec3, s1: vec3, s2: vec3) -> Self {
        Rect {
            origin,
            s1,
            s2,
            normal: s1.cross(s2).normalize(),
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
                let q1m = p.project_on(self.s1).mag() * -p.dot(self.s1).signum();
                let q2m = p.project_on(self.s2).mag() * -p.dot(self.s2).signum();
                if 0.0 <= q1m && q1m <= self.s1.mag() && 0.0 <= q2m && q2m <= self.s2.mag() {
                    return Some(Hit::new(ray, t, self.normal));
                }
            }
        }

        None
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        let min = self.origin.min(&(self.origin + self.s1 + self.s2)) - vec3::splat(0.0001);
        let max = self.origin.max(&(self.origin + self.s1 + self.s2)) + vec3::splat(0.0001);

        Some(AABB::new(min, max))
    }

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        if let Some(hit) = self.hit(&Ray::new(*origin, *dir), 0.001, f32::INFINITY) {
            let area = self.s1.mag() * self.s2.mag();
            let dist_sq = hit.t * hit.t * dir.mag_sq();
            let cosine = (dir.dot(hit.normal) / dir.mag()).abs();

            dist_sq / (cosine * area)
        } else {
            0.0
        }
    }

    fn random_point_on(&self) -> vec3 {
        self.origin + (self.s1 * rand::random::<f32>()) + (self.s2 * rand::random::<f32>())
    }
}
