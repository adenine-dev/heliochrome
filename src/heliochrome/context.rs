use std::time::Instant;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::maths::*;
use crate::objects::{Hittable, HittableList};

#[cfg(feature = "multithread")]
use rayon::prelude::*;

pub struct Context {
    pub camera: Camera,

    hittables: HittableList,

    size: Size<u16>,
    image: Image,
    pixel_buffer: Vec<u32>,
}

impl Context {
    pub fn new(size: Size<u16>, camera: Camera) -> Self {
        Self {
            camera,
            hittables: HittableList { hittables: vec![] },
            size,
            image: Image::new(size),
            pixel_buffer: vec![0; size.width as usize * size.height as usize],
        }
    }

    pub fn add_hittable(&mut self, hittable: impl Hittable + 'static) {
        self.hittables.hittables.push(Box::new(hittable));
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
        let hit = self.hittables.hit(&ray, 0.0, 1000.0);

        if let Some(mut hit) = hit {
            if hit.normal.dot(ray.direction) > 0.0 {
                hit.normal = -hit.normal;
            }
            ((hit.normal + vec3::splat(1.0)) / 2.0).into()
        } else {
            let t = (ray.direction.y + 1.0) / 2.0;
            (1.0 - t) * Color::new(0.0, 0.0, 0.0) + t * Color::new(0.1, 0.05, 0.15)
        }
    }

    pub fn render(&mut self) -> &Vec<u32> {
        let start = Instant::now();

        let per_pixel = |(i, _)| {
            let x = i % self.image.size.width as usize;
            let y = i / self.image.size.width as usize;
            let uv = vec2::new(
                x as f32 / (self.size.width - 1) as f32,
                1.0 - (y as f32 / (self.size.height - 1) as f32), // flip
            );

            self.render_fragment(&uv)
        };

        #[cfg(feature = "multithread")]
        {
            self.image.buffer = self
                .image
                .buffer
                .par_iter()
                .enumerate()
                .map(per_pixel)
                .collect();
        }

        #[cfg(not(feature = "multithread"))]
        {
            self.image.buffer = self
                .image
                .buffer
                .iter()
                .enumerate()
                .map(per_pixel)
                .collect();
        }

        self.pixel_buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                let out_color = self.image.buffer[i] * 255.999;
                let red = (out_color.r).floor() as u32;
                let green = (out_color.g).floor() as u32;
                let blue = (out_color.b).floor() as u32;

                *color = blue | (green << 8) | (red << 16)
            });

        let elapsed = start.elapsed();
        println!("frame render time: {elapsed:?}");
        &self.pixel_buffer
    }
}
