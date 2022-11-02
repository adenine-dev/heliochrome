use enum_dispatch::enum_dispatch;

use crate::{maths::vec3, maths::Ray};

pub struct BounceInfo {
    pub t: f32,
    pub p: vec3,
    pub normal: vec3,
    pub front_face: bool,
}

impl BounceInfo {
    pub fn new(src_ray: &Ray, t: f32, normal: vec3) -> Self {
        let front_face = src_ray.direction.dot(normal) < 0.0;

        Self {
            t,
            p: src_ray.at(t),
            normal: if front_face { normal } else { -normal },
            front_face,
        }
    }

    pub fn set_normal(&mut self, src_ray: &Ray, normal: vec3) {
        self.front_face = src_ray.direction.dot(normal) < 0.0;
        self.normal = if src_ray.direction.dot(normal) < 0.0 {
            normal
        } else {
            -normal
        };
    }
}

pub struct Intersection {
    pub t: f32,
    pub i: u32,
}

#[enum_dispatch]
#[allow(unused_variables)] // default trait impls
pub trait Hittable: Send + Sync + Clone {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo;

    fn make_bounding_box(&self) -> AABB;

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        panic!("oof you need to implement this :<");
    }

    fn random(&self, origin: &vec3) -> vec3 {
        panic!("haha you need to implement this :<");
    }
}

mod sphere;
pub use sphere::*;

mod infinite_plane;
pub use infinite_plane::*;

mod rect;
pub use rect::*;

mod triangle;
pub use triangle::*;

mod mesh;
pub use mesh::*;

mod aabb;
pub use aabb::*;

mod sdf;
pub use sdf::*;

#[enum_dispatch(Hittable)]
#[derive(Clone)]
pub enum HittableObject {
    Sphere,
    InfinitePlane,
    Rect,
    Triangle,
    Mesh,
    AABB,
    HittableSDF,
}
