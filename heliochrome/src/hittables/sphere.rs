use super::{Intersect, AABB};
use crate::{
    hittables::{Hit, Hittable},
    maths::*,
};

#[derive(Clone)]
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
        let intersection = self.intersect(ray, t_min, t_max)?;

        Some(Hit::new(
            ray,
            intersection.t,
            (ray.at(intersection.t) - self.center) / self.radius,
        ))
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersect> {
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

        Some(Intersect { t: root, i: 0 })
    }

    fn make_bounding_box(&self) -> AABB {
        AABB::new(
            self.center - vec3::splat(self.radius.abs()),
            self.center + vec3::splat(self.radius.abs()),
        )
    }

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        if self
            .hit(&Ray::new(*origin, *dir), 0.001, f32::INFINITY)
            .is_none()
        {
            0.0
        } else {
            let cos_theta_max =
                (1.0 - self.radius * self.radius / (self.center - origin).mag_sq()).sqrt();
            let solid_angle = std::f32::consts::TAU * (1.0 - cos_theta_max);

            1.0 / solid_angle
        }
    }

    fn random(&self, origin: &vec3) -> vec3 {
        let direction = self.center - origin;
        let distance_squared = direction.mag_sq();
        let uvw = ONB::new_from_w(direction);
        uvw.local(&vec3::random_to_sphere(self.radius, distance_squared))
    }
}
