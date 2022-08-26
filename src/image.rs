use crate::color::Color;
use crate::maths::Size;

pub struct Image {
    pub size: Size<u16>,
    pub buffer: Vec<f32>,
}

impl Image {
    pub fn new(size: Size<u16>) -> Image {
        Image {
            size,
            buffer: vec![0.0; 3 * size.width as usize * size.height as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: Color) {
        self.buffer[(y as usize * self.size.width as usize + x as usize) * 3 + 0] = color.r;
        self.buffer[(y as usize * self.size.width as usize + x as usize) * 3 + 1] = color.g;
        self.buffer[(y as usize * self.size.width as usize + x as usize) * 3 + 2] = color.b;
    }
}
