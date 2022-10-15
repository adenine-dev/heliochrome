use crate::{
    hittables::{Hit, Hittable, HittableObject, AABB},
    materials::Material,
    maths::vec3,
    maths::Ray,
    transform::Transform,
};

#[derive(Clone)]
pub struct Object {
    pub hittable: HittableObject,
    pub material: Material,
    pub transform: Option<Transform>,
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
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let r = if let Some(transform) = &self.transform {
            transform.trans_ray(ray)
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

    fn make_bounding_box(&self) -> AABB {
        let aabb = self.hittable.make_bounding_box();
        if let Some(transform) = &self.transform {
            return transform.trans_aabb(&aabb);
        }

        aabb
    }

    fn pdf_value(&self, origin: &vec3, dir: &vec3) -> f32 {
        if let Some(transform) = &self.transform {
            let origin = &transform.trans_pos(origin);
            let dir = &transform.trans_dir(dir);
            self.hittable.pdf_value(origin, dir)
        } else {
            self.hittable.pdf_value(origin, dir)
        }
    }

    fn random_point_on(&self) -> vec3 {
        let p = self.hittable.random_point_on();
        if let Some(transform) = &self.transform {
            // (transform.matrix * vec4::from_vec3(p, 1.0)).to_vec3()
            transform.trans_point(&p)
        } else {
            p
        }
    }
}
