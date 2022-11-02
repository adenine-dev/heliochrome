use super::{Hit, Hittable, Intersect, AABB};
use crate::maths::{vec3, Ray, ONB};

#[derive(Clone)]
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let intersection = self.intersect(ray, t_min, t_max)?;
        Some(Hit::new(ray, intersection.t, self.normal))
    }

    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersect> {
        let d = self.normal.dot(ray.direction);
        if d.abs() >= 0.0001 {
            let t = (self.origin - ray.origin).dot(self.normal) / d;
            if t_min <= t && t <= t_max {
                return Some(Intersect { t, i: 0 });
            }
        }

        None
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
