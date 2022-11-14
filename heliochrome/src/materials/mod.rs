use std::error::Error;

use enum_dispatch::enum_dispatch;

mod scatterable;
pub use scatterable::*;

mod lambertian;
pub use lambertian::*;

mod metal;
pub use metal::*;

mod dielectric;
pub use dielectric::*;

mod diffuse_light;
pub use diffuse_light::*;

use crate::{color::Color, hittables::BounceInfo, loader::FromHCY, maths::Ray};

#[enum_dispatch(Scatterable)]
#[derive(Clone, Debug)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}

impl FromHCY for Material {
    fn from_hcy(member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let member = member.ok_or("invalid syntax missing member specifier")?;
        match member {
            "lambertian" => Ok(Material::Lambertian(Lambertian::from_hcy(None, lines)?)),
            "metal" => Ok(Material::Metal(Metal::from_hcy(None, lines)?)),
            "dielectric" => Ok(Material::Dielectric(Dielectric::from_hcy(None, lines)?)),
            "diffuse_light" => Ok(Material::DiffuseLight(DiffuseLight::from_hcy(None, lines)?)),
            _ => Err(format!("unknown material {member}"))?,
        }
    }
}
