use super::{BounceInfo, Hittable, Intersection, AABB};
use crate::maths::{vec3, Ray};

#[derive(Clone, Default)]
pub struct Triangle {
    pub vertices: [vec3; 3],
}

impl Triangle {
    pub fn new(vertices: [vec3; 3]) -> Self {
        Self { vertices }
    }

    pub fn normal(&self) -> vec3 {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        edge2.cross(edge1).normalize()
    }
}

impl Hittable for Triangle {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);
        if a > -f32::EPSILON && a < f32::EPSILON {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin - self.vertices[0];
        let u = f * s.dot(h);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(q);
        if t < t_min || t > t_max {
            return None;
        }

        Some(Intersection { t, i: 0 })
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        BounceInfo::new(ray, intersection.t, self.normal())
    }

    fn make_bounding_box(&self) -> AABB {
        let mut min = vec3::splat(f32::INFINITY);
        let mut max = vec3::splat(-f32::INFINITY);
        for v in &self.vertices {
            min = min.min(v);
            max = max.max(v);
        }

        AABB::new(min - vec3::splat(0.0001), max + vec3::splat(0.0001))
    }
}
