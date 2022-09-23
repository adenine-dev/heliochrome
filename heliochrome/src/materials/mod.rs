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

use super::color::Color;
use super::hittables::Hit;
use super::maths::Ray;

#[enum_dispatch(Scatterable)]
pub enum Material {
    Lambertian,
    Metal,
    Dielectric,
    DiffuseLight,
}
