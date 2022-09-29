use crate::{color::Color, maths::vec3};

use super::{
    hittables::{Hit, Hittable, HittableObject, AABB},
    materials::{Material, Scatter, Scatterable},
    maths::Ray,
    transform::Transform,
};

#[derive(Clone)]
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
            let mut min = vec3::splat(f32::INFINITY);
            let mut max = vec3::splat(-f32::INFINITY);
            for i in 0..=1 {
                for j in 0..=1 {
                    for k in 0..=1 {
                        let p = transform.matrix
                            * (aabb.min
                                * vec3::new((1 - i) as f32, (1 - j) as f32, (1 - k) as f32)
                                + aabb.max * vec3::new(i as f32, j as f32, k as f32));
                        min = min.min(&p);
                        max = max.max(&p);
                    }
                }
            }

            return Some(AABB::new(min, max));
        }

        Some(aabb)
    }
}
