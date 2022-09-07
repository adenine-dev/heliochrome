use std::error::Error;
use std::fs::File;
use std::path::Path;

use crate::color::Color;
use crate::maths::Size;

use super::maths::vec2;

pub struct Image {
    pub size: Size<u16>,
    pub buffer: Vec<Color>,
}

impl Image {
    pub fn new(size: Size<u16>) -> Image {
        Image {
            size,
            buffer: vec![Color::splat(0.0); size.width as usize * size.height as usize],
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
                size: Size::new(info.width as u16, info.height as u16),
                buffer,
            });
        }

        Err("could not load :<")?
    }

    pub fn sample_uv(&self, uv: &vec2) -> Color {
        self.buffer[((uv.y * (self.size.height - 1) as f32).floor() as usize)
            * self.size.width as usize
            + ((uv.x * (self.size.width - 1) as f32).floor() as usize)]
    }
}
