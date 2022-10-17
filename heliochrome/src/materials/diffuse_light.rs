use super::Scatter;
use crate::{color::Color, hittables::Hit, materials::Scatterable, maths::Ray};

#[derive(Clone)]
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
    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<Scatter> {
        None
    }

    fn emitted(&self, hit: &Hit) -> Color {
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
