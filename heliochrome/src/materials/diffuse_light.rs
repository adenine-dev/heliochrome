use std::error::Error;

use super::Scatter;
use crate::{
    color::Color,
    hittables::BounceInfo,
    loader::{parse_into, FromHCY},
    materials::Scatterable,
    maths::Ray,
};

#[derive(Clone, Debug)]
pub struct DiffuseLight {
    color: Color,
}

impl DiffuseLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        Self {
            color: color * intensity,
        }
    }
}

impl Scatterable for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &BounceInfo) -> Option<Scatter> {
        None
    }

    fn emitted(&self, hit: &BounceInfo) -> Color {
        if hit.front_face {
            self.color
        } else {
            Color::splat(0.0)
        }
    }

    fn is_important(&self) -> bool {
        true
    }
}

impl FromHCY for DiffuseLight {
    fn from_hcy(_member: Option<&str>, lines: Vec<String>) -> Result<Self, Box<dyn Error>> {
        let mut color = None;
        let mut intensity = None;

        for line in lines.into_iter() {
            let (key, value) = line
                .split_once(':')
                .ok_or("invalid key value pair syntax")?;
            match key.trim() {
                "color" => {
                    color = Some(parse_into(value)?);
                }
                "intensity" => {
                    intensity = Some(parse_into(value)?);
                }
                _ => {}
            }
        }

        Ok(Self::new(
            color.ok_or("missing required key `color`")?,
            intensity.ok_or("missing required key `intensity`")?,
        ))
    }
}
