use crate::heliochrome::{
    color::Color,
    hittables::Hit,
    materials::{Scatter, Scatterable},
    maths::{vec3, Ray},
};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        Self { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = ray.direction.reflect_over(hit.normal);
        if reflected.dot(hit.normal) > 0.0 {
            Some(Scatter {
                outgoing: Ray::new(
                    ray.at(hit.t),
                    reflected + self.fuzz * vec3::random_in_unit_sphere(),
                ),
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}