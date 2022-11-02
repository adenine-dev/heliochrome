use enum_dispatch::enum_dispatch;

use crate::{color::Color, hittables::BounceInfo, maths::Ray, pdf::Pdf};

pub enum ScatterType<'a> {
    Pdf(Pdf<'a>),
    Specular(Ray),
}

pub struct Scatter<'a> {
    pub attenuation: Color,
    pub scatter_type: ScatterType<'a>,
}

#[enum_dispatch]
#[allow(unused_variables)] // default trait impls
pub trait Scatterable: Clone {
    fn scatter(&self, ray: &Ray, hit: &BounceInfo) -> Option<Scatter>;

    fn pdf(&self, incoming: &Ray, outgoing: &Ray, hit: &BounceInfo) -> f32 {
        0.0
    }

    fn emitted(&self, hit: &BounceInfo) -> Color {
        Color::splat(0.0)
    }

    fn is_important(&self) -> bool {
        false
    }
}
