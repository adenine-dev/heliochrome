use super::{Hit, Hittable, Intersect, AABB};
use crate::maths::{vec3, Ray};

#[derive(Clone, Default)]
pub struct Triangle {
    pub vertices: [vec3; 3],
}

impl Triangle {
    pub fn new(vertices: [vec3; 3]) -> Self {
        Self { vertices }
    }
}

pub(crate) fn triangle_bounding_box(vertices: &[vec3; 3]) -> AABB {
    let mut min = vec3::splat(f32::INFINITY);
    let mut max = vec3::splat(-f32::INFINITY);
    for v in vertices {
        min = min.min(v);
        max = max.max(v);
    }

    AABB::new(min - vec3::splat(0.0001), max + vec3::splat(0.0001))
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let intersection = self.intersect(ray, t_min, t_max)?;
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];
        Some(Hit::new(
            ray,
            intersection.t,
            edge2.cross(edge1).normalize(),
        ))
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersect> {
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

        Some(Intersect { t, i: 0 })
    }

    fn make_bounding_box(&self) -> AABB {
        triangle_bounding_box(&self.vertices)
    }
}
