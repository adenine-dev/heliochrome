use super::{
    hittables::{Hit, Hittable, HittableObject, AABB},
    materials::{Material, Scatter, Scatterable},
    maths::Ray,
    transform::Transform,
};
use crate::{color::Color, maths::vec3};

#[derive(Clone)]
pub struct Object {
    hittable: HittableObject,
    material: Material,
    transform: Option<Transform>,
}

impl Object {
    pub fn new<H: Into<HittableObject>, M: Into<Material>>(
        hittable: H,
        material: M,
        transform: Option<Transform>,
    ) -> Object {
        Object {
            hittable: hittable.into(),
            material: material.into(),
            transform,
        }
    }

    pub fn get_scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        self.material.scatter(ray, hit)
    }

    pub fn get_emitted(&self) -> Color {
        self.material.emitted()
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let r = if let Some(transform) = &self.transform {
            Ray::new(
                transform.trans_pos(&ray.origin),
                transform.trans_dir(&ray.direction),
            )
        } else {
            *ray
        };

        let mut hit = self.hittable.hit(&r, t_min, t_max);
        if let Some(hit) = &mut hit {
            if let Some(transform) = &self.transform {
                hit.p = ray.at(hit.t);
                hit.set_normal(ray, transform.trans_normal(&hit.normal));
            }
        }

        hit
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        let aabb = self.hittable.make_bounding_box()?;
        if let Some(transform) = &self.transform {
            return Some(transform.trans_aabb(&aabb));
        }

        Some(aabb)
    }
}
