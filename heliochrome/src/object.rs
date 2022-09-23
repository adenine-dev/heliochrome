use crate::color::Color;

use super::{
    hittables::{Hit, Hittable, HittableObject, AABB},
    materials::{Material, Scatter, Scatterable},
    maths::Ray,
    transform::Transform,
};

pub struct Object {
    hittable: HittableObject,
    material: Material,
    transform: Option<Transform>,
}

impl Object {
    pub fn new(
        hittable: HittableObject,
        material: Material,
        transform: Option<Transform>,
    ) -> Object {
        Object {
            hittable,
            material,
            transform,
        }
    }

    pub fn get_scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        self.material.scatter(&ray, hit)
    }

    pub fn get_emitted(&self) -> Color {
        self.material.emitted()
    }
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let r = if let Some(transform) = &self.transform {
            Ray::new(
                transform.inverse * ray.origin,
                transform.inverse * ray.direction,
            )
        } else {
            *ray
        };

        let mut hit = self.hittable.hit(&r, t_min, t_max);
        if let Some(hit) = &mut hit {
            if let Some(transform) = &self.transform {
                hit.p = ray.at(hit.t);
                hit.set_normal(&ray, (transform.normal_matrix * hit.normal).normalize());
            }
        }

        hit
    }

    fn make_bounding_box(&self) -> Option<AABB> {
        let aabb = self.hittable.make_bounding_box()?;
        if let Some(transform) = &self.transform {
            let mut min = transform.matrix * aabb.min;
            let mut max = transform.matrix * aabb.max;
            min = min.min(&max);
            max = max.max(&min);

            return Some(AABB::new(min, max));
        }

        Some(aabb)
    }
}
