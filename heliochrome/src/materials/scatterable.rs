use enum_dispatch::enum_dispatch;

use crate::{color::Color, hittables::Hit, maths::Ray};

pub struct Scatter {
    pub outgoing: Ray,
    pub attenuation: Color,
    pub pdf: f32,
}

#[enum_dispatch]
pub trait Scatterable: Clone {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;

    fn pdf(&self, incoming: &Ray, outgoing: &Ray, hit: &Hit) -> f32 {
        0.0
    }

    fn emitted(&self, hit: &Hit) -> Color {
        Color::splat(0.0)
    }

    fn is_light(&self) -> bool {
        false
    }
}
