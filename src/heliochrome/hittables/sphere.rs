use crate::{
    hittables::{Hit, Hittable},
    materials::Material,
    maths::*,
};

pub struct Sphere {
    pub center: vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
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

        Some(Hit {
            t: root,
            normal: ((ray.at(root) - self.center) / self.radius),
            material: &self.material,
        })
    }
}
