use crate::{
    hittables::{Hit, Hittable},
    maths::*,
};

use super::AABB;

pub struct Sphere {
    pub center: vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.mag_sq();
        let half_b = oc.dot(ray.direction);
        let c = oc.mag_sq() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        Some(Hit::new(
            ray,
            root,
            (ray.at(root) - self.center) / self.radius,
        ))
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.center - vec3::splat(self.radius),
            self.center + vec3::splat(self.radius),
        ))
    }
}
