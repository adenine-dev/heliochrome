use std::error::Error;

use crate::{
    hittables::{BounceInfo, Hittable, HittableObject, Intersection, AABB},
    loader::{collect_until_next_item, FromHCY},
    materials::Material,
    maths::vec3,
    maths::Ray,
    transform::Transform,
};

#[derive(Clone, Debug)]
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
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let r = if let Some(transform) = &self.transform {
            transform.trans_ray(ray)
        } else {
            *ray
        };

        self.hittable.intersect(&r, t_min, t_max)
    }

    fn get_bounce_info(&self, ray: &Ray, intersection: Intersection) -> BounceInfo {
        let r = if let Some(transform) = &self.transform {
            transform.trans_ray(ray)
        } else {
            *ray
        };

        let mut bounce_info = self.hittable.get_bounce_info(&r, intersection);
        if let Some(transform) = &self.transform {
            bounce_info.p = ray.at(bounce_info.t);
            bounce_info.set_normal(ray, transform.trans_normal(&bounce_info.normal));
        }

        bounce_info
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

    fn random(&self, origin: &vec3) -> vec3 {
        if let Some(transform) = &self.transform {
            let origin = &transform.trans_pos(origin);
            self.hittable.random(origin)
        } else {
            self.hittable.random(origin)
        }
    }
}

impl FromHCY for Object {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut hittable = None;
        let mut material = None;
        let mut transform = None;

        let mut line_iter = lines.iter();
        while let Some(line) = line_iter.next() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "primitive" => {
                    hittable = Some(
                        HittableObject::from_hcy(
                            Some(value.trim()),
                            collect_until_next_item(&mut line_iter),
                        )
                        .map_err(|err| format!("could not parse primitive key: {err}"))?,
                    );
                }
                "material" => {
                    material = Some(
                        Material::from_hcy(
                            Some(value.trim()),
                            collect_until_next_item(&mut line_iter),
                        )
                        .map_err(|err| format!("could not parse material key: {err}"))?,
                    );
                }
                "transform" => {
                    transform = Some(
                        Transform::from_hcy(None, collect_until_next_item(&mut line_iter))
                            .map_err(|err| format!("could not parse transform key: {err}"))?,
                    )
                }
                _ => {}
            }
        }

        Ok(Object::new(
            hittable.ok_or("missing required object parameter `primitive`")?,
            material.ok_or("missing required object parameter `material`")?,
            transform,
        ))
    }
}
