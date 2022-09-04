use crate::heliochrome::{
    color::Color,
    hittables::Hit,
    materials::Scatterable,
    maths::{vec3, Ray},
};

use super::Scatter;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut dir = hit.normal + vec3::random_in_unit_sphere().normalize();
        if dir.near_zero() {
            dir = hit.normal;
        }
        Some(Scatter {
            outgoing: Ray::new(ray.at(hit.t), dir),
            attenuation: self.albedo,
        })
    }
}
