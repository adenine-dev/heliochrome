use enum_dispatch::enum_dispatch;

mod hittable;
pub use hittable::*;

mod sphere;
pub use sphere::*;

mod hittable_list;
pub use hittable_list::*;

use crate::maths::Ray;

#[enum_dispatch(Hittable)]
pub enum HittableObject {
    Sphere,
    HittableList,
}
