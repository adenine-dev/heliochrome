use super::{
    hittables::{Hit, Hittable, HittableObject},
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

    pub fn get_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let r = if let Some(transform) = &self.transform {
            Ray::new(
                transform.matrix * ray.origin,
                transform.matrix * ray.direction,
            )
        } else {
            *ray
        };

        self.hittable.hit(&r, t_min, t_max)
    }

    pub fn get_scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        self.material.scatter(ray, hit)
    }
}
