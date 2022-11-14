use std::error::Error;

use super::{Intersection, AABB};
use crate::{
    hittables::{BounceInfo, Hittable},
    loader::{parse_into, FromHCY},
    maths::*,
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

pub fn random_to_sphere(radius: f32, dist_sq: f32) -> vec3 {
    let r1 = rand::random::<f32>();
    let r2 = rand::random::<f32>();
    let z = 1.0 + r2 * ((1.0 - radius * radius / dist_sq).sqrt() - 1.0);

    let phi = std::f32::consts::TAU * r1;
    let x = phi.cos() * (1.0 - z * z).sqrt();
    let y = phi.sin() * (1.0 - z * z).sqrt();

    vec3::new(x, y, z)
}

impl Hittable for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
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

        Some(Intersection { t: root, i: 0 })
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        BounceInfo::new(
            ray,
            intersection.t,
            (ray.at(intersection.t) - self.center) / self.radius,
        )
    }

    fn make_bounding_box(&self) -> AABB {
        AABB::new(
            self.center - vec3::splat(self.radius.abs()),
            self.center + vec3::splat(self.radius.abs()),
        )
    }

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        if self
            .intersect(&Ray::new(*origin, *dir), 0.001, f32::INFINITY)
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
        uvw.local(&random_to_sphere(self.radius, distance_squared))
    }
}

impl FromHCY for Sphere {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut origin = None;
        let mut radius = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "origin" => origin = Some(parse_into(value)?),
                "radius" => radius = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(Sphere::new(
            origin.ok_or("missing required key `origin`")?,
            radius.ok_or("missing required key `radius`")?,
        ))
    }
}
