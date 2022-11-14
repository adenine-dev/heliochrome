use std::fmt::Debug;

use crate::{hittables::AABB, maths::vec3};

const NORMAL_H: f32 = 0.0001;

pub trait SDF: Send + Sync + Debug {
    fn dist(&self, p: vec3) -> f32;

    fn normal_at(&self, p: &vec3) -> vec3 {
        // let ks = [
        //     vec3::new(1.0, -1.0, -1.0),
        //     vec3::new(-1.0, -1.0, 1.0),
        //     vec3::new(-1.0, 1.0, -1.0),
        //     vec3::splat(1.0),
        // ];
        // ks.into_iter()
        //     .fold(vec3::splat(0.0), |a, c| {
        //         a + (c * self.dist(p + c * NORMAL_H))
        //     })
        //     .normalize()

        (vec3::new(
            self.dist(p + vec3::unit_x() * NORMAL_H) - self.dist(p - vec3::unit_x() * NORMAL_H),
            self.dist(p + vec3::unit_y() * NORMAL_H) - self.dist(p - vec3::unit_y() * NORMAL_H),
            self.dist(p + vec3::unit_z() * NORMAL_H) - self.dist(p - vec3::unit_z() * NORMAL_H),
        ))
        .normalized()
    }

    fn make_bounding_box(&self) -> AABB;

    fn difference<T: SDF>(self, other: T) -> Difference<Self, T>
    where
        Self: Sized,
    {
        Difference { a: self, b: other }
    }

    fn smooth_difference<T: SDF>(self, k: f32, other: T) -> SmoothDifference<Self, T>
    where
        Self: Sized,
    {
        SmoothDifference {
            k,
            a: self,
            b: other,
        }
    }

    fn intersection<T: SDF>(self, other: T) -> Intersection<Self, T>
    where
        Self: Sized,
    {
        Intersection { a: self, b: other }
    }

    fn smooth_intersection<T: SDF>(self, k: f32, other: T) -> SmoothIntersection<Self, T>
    where
        Self: Sized,
    {
        SmoothIntersection {
            k,
            a: self,
            b: other,
        }
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

    fn modulo(self, period: vec3) -> Modulo<Self>
    where
        Self: Sized,
    {
        Modulo {
            period,
            primitive: self,
        }
    }
}

mod operations;
pub use operations::*;
mod primitives;
pub use primitives::*;
