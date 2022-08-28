use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::maths::*;

pub struct Context {
    pub camera: Camera,

    size: Size<u16>,
    image: Image,
    pixel_buffer: Vec<u32>,
}

impl Context {
    pub fn new(size: Size<u16>, camera: Camera) -> Self {
        Self {
            camera,
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
        self.camera = Camera::new(self.camera.eye, size.width as f32 / size.height as f32);
    }

    pub fn render_fragment(&self, uv: &vec2) -> Color {
        let ray = self.camera.get_ray(uv);

        let hit = {
            let center = vec3::new(0.0, 0.0, 1.0);
            let radius = 0.5;
            let oc = ray.origin - center;
            let a = ray.direction.dot(ray.direction);
            let b = 2.0 * oc.dot(ray.direction);
            let c = oc.dot(oc) - radius * radius;
            let discriminant = b * b - 4.0 * a * c;
            discriminant > 0.0
        };

        if hit {
            Color::new(1.0, 0.0, 0.0)
        } else {
            let t = (ray.direction.y + 1.0) / 2.0;
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn render(&mut self) -> &Vec<u32> {
        for y in 0..self.size.height {
            for x in 0..self.size.width {
                let uv = vec2::new(
                    x as f32 / (self.size.width - 1) as f32,
                    y as f32 / (self.size.height - 1) as f32,
                );

                self.image.set_pixel(x, y, self.render_fragment(&uv))
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
