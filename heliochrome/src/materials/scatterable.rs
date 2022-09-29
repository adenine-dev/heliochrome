use enum_dispatch::enum_dispatch;

use crate::{color::Color, hittables::Hit, maths::Ray};

pub struct Scatter {
    pub outgoing: Ray,
    pub attenuation: Color,
}

#[enum_dispatch]
pub trait Scatterable: Clone {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;

    fn emitted(&self) -> Color {
        Color::splat(0.0)
    }
}
