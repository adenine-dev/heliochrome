use crate::color::Color;
use crate::maths::Size;

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
}
