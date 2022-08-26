use crate::color::Color;
use crate::image::Image;
use crate::maths::Size;

pub struct Context {
    size: Size<u16>,
    image: Image,

    pixel_buffer: Vec<u32>,
}

impl Context {
    pub fn new(size: Size<u16>) -> Self {
        Self {
            size,
            image: Image::new(size),
            pixel_buffer: vec![0; size.width as usize * size.height as usize],
        }
    }

    pub fn get_size(&self) -> Size<u16> {
        self.image.size
    }

    pub fn get_pixel_buffer(&self) -> &[u32] {
        &self.pixel_buffer
    }

    pub fn resize(&mut self, size: Size<u16>) {
        self.size = size;
        self.image = Image::new(size);
        self.pixel_buffer = vec![0; size.width as usize * size.height as usize];
    }

    pub fn trace(&self, u: f32, v: f32) -> Color {
        Color { r: u, g: 0.0, b: v }
    }

    pub fn render(&mut self) -> &Vec<u32> {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                self.image.set_pixel(
                    x,
                    y,
                    self.trace(
                        x as f32 / self.size.width as f32,
                        y as f32 / self.size.height as f32,
                    ),
                )
            }
        }

        self.pixel_buffer = self
            .pixel_buffer
            .iter()
            .enumerate()
            .map(|(idx, _)| {
                let red = (self.image.buffer[idx * 3 + 0] * 255.999).floor() as u32;
                let green = (self.image.buffer[idx * 3 + 1] * 255.999).floor() as u32;
                let blue = (self.image.buffer[idx * 3 + 2] * 255.999).floor() as u32;

                blue | (green << 8) | (red << 16)
            })
            .collect::<Vec<_>>();

        &self.pixel_buffer
    }
}
