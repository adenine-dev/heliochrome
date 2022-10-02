use std::sync::Arc;

use super::{Hit, Hittable, AABB};
use crate::maths::{lerp, vec3, Ray};

const NORMAL_H: f32 = 0.000001;
const MIN_DIST: f32 = 0.000001;
const MAX_MARCHES: u16 = 500;
const MARCH_T_MAX: f32 = 10000.0;

pub trait SDF: Send + Sync {
    fn dist(&self, p: vec3) -> f32;

    fn normal_at(&self, p: &vec3) -> vec3 {
        (vec3::new(
            self.dist(p + vec3::unit_x() * NORMAL_H) - self.dist(p - vec3::unit_x() * NORMAL_H),
            self.dist(p + vec3::unit_y() * NORMAL_H) - self.dist(p - vec3::unit_y() * NORMAL_H),
            self.dist(p + vec3::unit_z() * NORMAL_H) - self.dist(p - vec3::unit_z() * NORMAL_H),
        ))
        .normalized()
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        None
    }

    fn smooth_union<T: SDF>(self, k: f32, other: T) -> SmoothUnion<Self, T>
    where
        Self: Sized,
    {
        SmoothUnion {
            k,
            a: self,
            b: other,
        }
    }
}

#[derive(Clone)]
pub struct HittableSDF {
    sdf: Arc<dyn SDF>,
}

impl HittableSDF {
    pub fn new(de: impl SDF + 'static) -> Self {
        Self { sdf: Arc::new(de) }
    }
}

impl Hittable for HittableSDF {
    fn hit(&self, ray: &Ray, mut t_min: f32, t_max: f32) -> Option<Hit> {
        let mut p = ray.at(t_min);

        for _ in 0..MAX_MARCHES {
            let d = self.sdf.dist(p);
            t_min += d;
            if t_min > t_max || t_min > MARCH_T_MAX {
                break;
            }
            p = ray.at(t_min);
            if d < MIN_DIST {
                return Some(Hit::new(ray, t_min, self.sdf.normal_at(&p)));
            }
        }

        None
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        self.sdf.make_bounding_box()
    }
}

pub struct SmoothUnion<A: SDF, B: SDF> {
    pub k: f32,
    pub a: A,
    pub b: B,
}

impl<A: SDF, B: SDF> SDF for SmoothUnion<A, B> {
    fn dist(&self, p: vec3) -> f32 {
        let d1 = self.a.dist(p);
        let d2 = self.b.dist(p);
        // let h = (0.5 + 0.5 * (d2 - d1) / self.k).clamp(0.0, 1.0);
        // lerp(d2, d1, h) - self.k * h * (1.0 - h)

        let h = (self.k - (d1 - d2).abs()).max(0.0) / self.k;
        d1.min(d2) - h * h * self.k * (1.0 / 4.0)
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        if let Some(a) = self.a.make_bounding_box() {
            if let Some(b) = self.b.make_bounding_box() {
                return Some(AABB::new(
                    a.min.min(&b.min) - vec3::splat(self.k - 1.0),
                    a.max.max(&b.max) + vec3::splat(self.k - 1.0),
                ));
            }
        }

        None
    }
}

pub struct SphereSDF {
    pub r: f32,
    pub c: vec3,
}

impl SphereSDF {
    pub fn new(r: f32, c: vec3) -> Self {
        Self { r, c }
    }
}

impl SDF for SphereSDF {
    fn dist(&self, p: vec3) -> f32 {
        (p - self.c).mag() - self.r
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        Some(AABB::new(
            self.c - vec3::splat(self.r),
            self.c + vec3::splat(self.r),
        ))
    }
}
