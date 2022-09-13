use crate::heliochrome::maths::{vec3, Ray};

use super::{Hit, Hittable};

pub struct Triangle {
    vertices: [vec3; 3],
}

impl Triangle {
    pub fn new(vertices: [vec3; 3]) -> Self {
        Self { vertices }
    }
}

pub(crate) fn ray_triangle_intersection(
    ray: &Ray,
    vertices: &[vec3; 3],
    t_min: f32,
    t_max: f32,
) -> Option<Hit> {
    let edge1 = vertices[1] - vertices[0];
    let edge2 = vertices[2] - vertices[0];
    let h = ray.direction.cross(edge2);
    let a = edge1.dot(h);
    if a > -f32::EPSILON && a < f32::EPSILON {
        return None;
    }
    let f = 1.0 / a;
    let s = ray.origin - vertices[0];
    let u = f * s.dot(h);
    if u < 0.0 || u > 1.0 {
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

    Some(Hit::new(ray, t, edge1.cross(edge2).normalize()))
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        ray_triangle_intersection(ray, &self.vertices, t_min, t_max)
    }
}
