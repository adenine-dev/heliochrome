use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::color::Color;

use super::maths::vec2;

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
        stb::image::stbi_set_flip_vertically_on_load(true);
        let mut f = File::open(path)?;

        let res = stb::image::stbi_loadf_from_reader(&mut f, stb::image::Channels::Rgb);
        if let Some((info, data)) = res {
            let buffer = (0..((info.width * info.height) as usize))
                .map(|i| {
                    Color::new(
                        data.as_slice()[i * 3 + 0].clamp(0.0, 1.0),
                        data.as_slice()[i * 3 + 1].clamp(0.0, 1.0),
                        data.as_slice()[i * 3 + 2].clamp(0.0, 1.0),
                    )
                })
                .collect::<Vec<Color>>();

            return Ok(Image {
                size: vec2::new(info.width as f32, info.height as f32),
                buffer,
            });
        }

        Err("could not load :<")?
    }

    pub fn sample_uv(&self, uv: &vec2) -> Color {
        //TODO: smooth sample
        self.buffer[((uv.y * (self.size.y - 1.0)).floor() as usize) * self.size.x as usize
            + ((uv.x * (self.size.x - 1.0)).floor() as usize)]
    }
}
