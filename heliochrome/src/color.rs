use std::str::FromStr;

use crate::maths::*;

vec3_impl!(Color, f32, f32, r, g, b);

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

    pub fn exp(&self) -> Self {
        Self::new(self.r.exp(), self.g.exp(), self.b.exp())
    }

    pub fn powf(&self, n: f32) -> Self {
        Self::new(self.r.powf(n), self.g.powf(n), self.b.powf(n))
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

impl FromStr for Color {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split(',').into_iter().collect::<Vec<_>>();
        if iter.len() == 3 {
            Ok(Color::new(
                iter[0].trim().parse().map_err(|x| format!("{x}"))?,
                iter[1].trim().parse().map_err(|x| format!("{x}"))?,
                iter[2].trim().parse().map_err(|x| format!("{x}"))?,
            ))
        } else {
            Err("invalid color string, unexpected number of components".to_owned())
        }
    }
}
