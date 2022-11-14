use std::error::Error;

use super::ScatterType;
use crate::{
    color::Color,
    hittables::BounceInfo,
    loader::{parse_into, FromHCY},
    materials::{Scatter, Scatterable},
    maths::{vec3, Ray},
};

#[derive(Clone, Debug)]
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
    fn scatter(&self, ray: &Ray, hit: &BounceInfo) -> Option<Scatter> {
        let reflected = ray.direction.reflect_over(hit.normal);
        if reflected.dot(hit.normal) > 0.0 {
            Some(Scatter {
                // outgoing: Ray::new(hit.p, reflected + self.fuzz * vec3::random_in_unit_sphere()),
                attenuation: self.albedo,
                scatter_type: ScatterType::Specular(Ray::new(
                    hit.p,
                    reflected + self.fuzz * vec3::random_in_unit_sphere(),
                )),
            })
        } else {
            None
        }
    }
}

impl FromHCY for Metal {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut albedo = None;
        let mut fuzz = Some(0.0);

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "albedo" => albedo = Some(parse_into(value)?),
                "fuzz" => fuzz = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(Metal::new(
            albedo.ok_or("missing required key `albedo`")?,
            fuzz.unwrap(),
        ))
    }
}
