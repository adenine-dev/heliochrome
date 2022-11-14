use std::error::Error;

use rand::random;

use super::ScatterType;
use crate::{
    color::Color,
    hittables::BounceInfo,
    loader::{parse_into, FromHCY},
    materials::{Scatter, Scatterable},
    maths::Ray,
};

#[derive(Clone, Debug)]
pub struct Dielectric {
    pub ir: f32,
    pub color: Color,
}

impl Dielectric {
    pub fn new(ir: f32, color: Color) -> Self {
        Self { ir, color }
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &BounceInfo) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = ray.direction.normalized();
        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_reflect = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_reflect || reflectance(cos_theta, refraction_ratio) > random() {
            unit_dir.reflect_over(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };

        Some(Scatter {
            attenuation: self.color,
            scatter_type: ScatterType::Specular(Ray::new(hit.p, direction)),
        })
    }

    fn is_important(&self) -> bool {
        false
    }
}

impl FromHCY for Dielectric {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut ir = None;
        let mut color = Some(Color::splat(1.0));

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "ir" => ir = Some(parse_into(value)?),
                "color" => color = Some(parse_into(value)?),
                _ => {}
            }
        }

        Ok(Dielectric::new(
            ir.ok_or("missing required key `ir`")?,
            color.unwrap(),
        ))
    }
}
