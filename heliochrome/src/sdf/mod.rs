use std::sync::atomic::AtomicU16;

use crate::{hittables::AABB, maths::vec3};

const NORMAL_H: f32 = f32::EPSILON;

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

    fn twist(self, k: f32) -> Twist<Self>
    where
        Self: Sized,
    {
        Twist { k, primitive: self }
    }
}

mod operations;
pub use operations::*;
mod primitives;
pub use primitives::*;
