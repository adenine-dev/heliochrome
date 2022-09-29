use crate::{
    color::Color,
    hittables::Hit,
    materials::Scatterable,
    maths::{vec3, Ray},
};

use super::Scatter;

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut dir = hit.normal + vec3::random_in_unit_sphere().normalize();
        if dir.near_zero() {
            dir = hit.normal;
        }
        Some(Scatter {
            outgoing: Ray::new(hit.p, dir),
            attenuation: self.albedo,
        })
    }
}
