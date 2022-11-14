use std::error::Error;

use super::{BounceInfo, Hittable, Intersection, AABB};
use crate::{
    loader::{parse_into, FromHCY},
    maths::{vec3, Ray, ONB},
};

#[derive(Clone, Debug)]
pub struct InfinitePlane {
    pub origin: vec3,
    pub normal: vec3,
}

impl InfinitePlane {
    pub fn new(origin: vec3, normal: vec3) -> Self {
        InfinitePlane { origin, normal }
    }
}

impl Hittable for InfinitePlane {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let d = self.normal.dot(ray.direction);
        if d.abs() >= 0.0001 {
            let t = (self.origin - ray.origin).dot(self.normal) / d;
            if t_min <= t && t <= t_max {
                return Some(Intersection { t, i: 0 });
            }
        }

        None
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        BounceInfo::new(ray, intersection.t, self.normal)
    }

    fn make_bounding_box(&self) -> AABB {
        // this lets the bounding box be tight if the normal is axis aligned.
        let onb = ONB::new_from_w(self.normal);
        let v = ((onb.v + onb.u) * f32::INFINITY).un_nan().abs();
        AABB::new(
            -v + self.origin - vec3::splat(0.0001),
            v + self.origin + vec3::splat(0.0001),
        )
    }
}

impl FromHCY for InfinitePlane {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut origin = None;
        let mut normal = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "origin" => origin = Some(parse_into(value)?),
                "normal" => normal = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(InfinitePlane::new(
            origin.ok_or("missing required key `origin`")?,
            normal.ok_or("missing required key `normal`")?,
        ))
    }
}
