use super::{BounceInfo, Hittable, Intersection, AABB};
use crate::maths::{mat3, vec3, Ray};

#[derive(Clone, Debug)]
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
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let b_inv = mat3::new([self.s1, self.s2, ray.direction]).inverse();
        let ol = b_inv * (ray.origin - self.origin);
        let t = -ol.z;
        if t_min <= t && t <= t_max && 0.0 <= ol.x && ol.x <= 1.0 && 0.0 <= ol.y && ol.y <= 1.0 {
            Some(Intersection { t, i: 0 })
        } else {
            None
        }
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        BounceInfo::new(ray, intersection.t, self.normal)
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
        if let Some(bounce) = self.intersect(&Ray::new(*origin, *dir), 0.001, f32::INFINITY) {
            let area = self.s1.mag() * self.s2.mag();
            let dist_sq = bounce.t * bounce.t * dir.mag_sq();
            let cosine = (dir.dot(self.normal).abs() / dir.mag()).abs();

            dist_sq / (cosine * area)
        } else {
            0.0
        }
        // 0.0
    }

    fn random(&self, origin: &vec3) -> vec3 {
        (self.origin + (self.s1 * rand::random::<f32>()) + (self.s2 * rand::random::<f32>())
            - origin)
            .normalize()
    }
}
