use crate::maths::*;

vec3_impl!(Color, f32, r, g, b);

impl Color {
    pub fn from_hex(hex: u32) -> Color {
        let r = hex & 0xff;
        let g = hex & 0x00ff;
        let b = hex & 0x0000ff;

        Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }

    pub fn luminance(&self) -> f32 {
        0.2126 * self.r + 0.7152 * self.g + 0.0722 * self.b
    }

    pub fn change_luminance(&self, nl: f32) -> Color {
        self * (nl / self.luminance())
    }
}

impl From<vec3> for Color {
    fn from(v: vec3) -> Self {
        Color::new(v.x, v.y, v.z)
    }
}

impl From<Color> for vec3 {
    fn from(color: Color) -> Self {
        vec3::new(color.r, color.g, color.b)
    }
}
