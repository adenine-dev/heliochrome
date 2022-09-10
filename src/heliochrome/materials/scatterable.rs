use enum_dispatch::enum_dispatch;

use crate::heliochrome::{color::Color, hittables::Hit, maths::Ray};

pub struct Scatter {
    pub outgoing: Ray,
    pub attenuation: Color,
}

#[enum_dispatch]
pub trait Scatterable {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
