use crate::{color::Color, hittables::Hit, materials::Scatterable, maths::Ray};

use super::Scatter;

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

    fn emitted(&self) -> Color {
        self.color
    }
}
