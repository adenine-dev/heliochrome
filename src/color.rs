use crate::maths::*;

vec3_impl!(Color, f32, r, g, b);

impl From<vec3> for Color {
    fn from(v: vec3) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}
