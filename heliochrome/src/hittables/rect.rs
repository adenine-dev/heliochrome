use super::{Hit, Hittable, AABB};
use crate::maths::{mat3, vec3, Ray};

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
        let b_inv = mat3::new([self.s1, self.s2, ray.direction]).inverse();
        let ol = b_inv * (ray.origin - self.origin);
        let t = -ol.z;
        if t_min <= t && t <= t_max && 0.0 <= ol.x && ol.x <= 1.0 && 0.0 <= ol.y && ol.y <= 1.0 {
            Some(Hit::new(ray, t, self.normal))
        } else {
            None
        }

        // works for rectangles but not parallelograms
        // let d = self.normal.dot(ray.direction);
        // if d.abs() >= 0.0001 {
        //     let t = (self.origin - ray.origin).dot(self.normal) / d;
        //     if t_min <= t && t <= t_max {
        //         let v = ray.at(t) - self.origin;
        //         let q1m = v.dot(self.s1);
        //         let q2m = v.dot(self.s2);
        //         if 0.0 <= q1m && q1m <= self.s1.mag_sq() && 0.0 <= q2m && q2m <= self.s2.mag_sq() {
        //             return Some(Hit::new(ray, t, self.normal));
        //         }
        //     }
        // }
    }

    fn make_bounding_box(&self) -> AABB {
        let min = self
            .origin
            .min(&(self.origin + self.s1))
            .min(&(self.origin + self.s2))
            .min(&(self.origin + self.s1 + self.s2))
            - vec3::splat(0.001);
        let max = self
            .origin
            .max(&(self.origin + self.s1))
            .max(&(self.origin + self.s2))
            .max(&(self.origin + self.s1 + self.s2))
            + vec3::splat(0.001);

        AABB::new(min, max)
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

    fn random(&self, origin: &vec3) -> vec3 {
        (self.origin + (self.s1 * rand::random::<f32>()) + (self.s2 * rand::random::<f32>())
            - origin)
            .normalize()
    }
}
