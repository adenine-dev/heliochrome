use enum_dispatch::enum_dispatch;

mod scatterable;
pub use scatterable::*;

mod lambertian;
pub use lambertian::*;

use super::hittables::Hit;
use super::maths::Ray;

#[enum_dispatch(Scatterable)]
pub enum Material {
    Lambertian,
}
