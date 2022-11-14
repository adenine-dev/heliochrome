use std::error::Error;
use std::path::Path;

use image;

use super::maths::vec2;
use crate::color::Color;

#[derive(Clone, Debug)]
pub struct Image {
    pub size: vec2,
    pub buffer: Vec<Color>,
}

impl Image {
    pub fn new(size: vec2) -> Image {
        Image {
            size,
            buffer: vec![Color::splat(0.0); size.x as usize * size.y as usize],
        }
    }

    pub fn load_from_hdri(path: &Path) -> Result<Image, Box<dyn Error>> {
        let img = image::open(path)?.flipv().into_rgb32f();

        let (width, height) = img.dimensions();

        return Ok(Image {
            size: vec2::new(width as f32, height as f32),
            buffer: img
                .enumerate_pixels()
                .map(|p| Color::new(p.2[0].min(1.0), p.2[1].min(1.0), p.2[2].min(1.0)))
                .collect::<Vec<_>>(),
        });
    }

    pub fn sample_uv(&self, uv: &vec2) -> Color {
        let x0 = (uv.x * (self.size.x - 1.0)).floor() as usize;
        let x1 = (uv.x * (self.size.x - 1.0)).ceil() as usize;
        let xx = (uv.x * (self.size.x - 1.0)).fract();
        let y0 = (uv.y * (self.size.y - 1.0)).floor() as usize;
        let y1 = (uv.y * (self.size.y - 1.0)).ceil() as usize;
        let yy = (uv.y * (self.size.y - 1.0)).fract();

        let c1 = self.buffer[y0 * self.size.x as usize + x0];
        let c2 = self.buffer[y0 * self.size.x as usize + x1];
        let c3 = self.buffer[y1 * self.size.x as usize + x0];
        let c4 = self.buffer[y1 * self.size.x as usize + x1];

        c1 * (1.0 - xx) * (1.0 - yy) + c2 * (xx) * (1.0 - yy) + c3 * (1.0 - xx) * yy + c4 * xx * yy
    }

    pub fn set_pixel(&mut self, pos: &vec2, color: Color) {
        self.buffer[(pos.x + pos.y * self.size.x) as usize] = color;
    }

    pub fn to_gamma_corrected_rgba8(&self, gamma: f32) -> Vec<u8> {
        self.buffer
            .iter()
            .flat_map(|c| {
                let out = c.powf(1.0 / gamma).clamp(0.0, 0.999) * 256.0;
                [out[0] as u8, out[1] as u8, out[2] as u8, 0xff]
            })
            .collect::<Vec<_>>()
    }
}
