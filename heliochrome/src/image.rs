use std::error::Error;
use std::path::Path;

use image;

use super::maths::vec2;
use crate::color::Color;

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
        //TODO: smooth sample
        self.buffer[((uv.y * (self.size.y - 1.0)).floor() as usize) * self.size.x as usize
            + ((uv.x * (self.size.x - 1.0)).floor() as usize)]
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
