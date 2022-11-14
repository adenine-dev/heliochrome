use std::error::Error;

use super::{Scatter, ScatterType};
use crate::{
    color::Color,
    hittables::BounceInfo,
    loader::{parse_into, FromHCY},
    materials::Scatterable,
    maths::Ray,
    pdf::CosinePdf,
};

#[derive(Clone, Debug)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &BounceInfo) -> Option<Scatter> {
        // let uvw = ONB::new_from_w(hit.normal);
        // let dir = uvw.local(&vec3::random_cosine_direction());
        Some(Scatter {
            // outgoing: Ray::new(hit.p, dir),
            attenuation: self.albedo,
            scatter_type: ScatterType::Pdf(CosinePdf::new(hit.normal).into()),
        })
    }

    fn pdf(&self, _incoming: &Ray, outgoing: &Ray, hit: &BounceInfo) -> f32 {
        let cosine = hit.normal.dot(outgoing.direction.normalized());
        if cosine > 0.0 {
            cosine / std::f32::consts::PI
        } else {
            0.0
        }
    }
}

impl FromHCY for Lambertian {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut albedo = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            if "albedo" == key.trim() {
                albedo = Some(parse_into(value)?)
            }
        }

        Ok(Lambertian::new(
            albedo.ok_or("missing required key `albedo`")?,
        ))
    }
}
