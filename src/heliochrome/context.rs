use rand::prelude::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittables::{Hittable, HittableList};
use crate::image::Image;
use crate::materials::*;
use crate::maths::*;

#[cfg(feature = "multithread")]
use rayon::prelude::*;

use super::hittables::HittableObject;

pub struct Context {
    pub camera: Camera,

    hittables: HittableList,

    size: Size<u16>,
    pub samples: u16,
    accumulated_image: Image,

    pixel_buffer: Vec<u32>,
}

const MAX_DEPTH: u32 = 50;

fn gamma_correct(color: Color, gamma: f32) -> Color {
    color.powf(1.0 / gamma)
}

impl Context {
    pub fn new(size: Size<u16>, camera: Camera) -> Self {
        Self {
            camera,
            hittables: HittableList { hittables: vec![] },
            size,
            samples: 0,
            accumulated_image: Image::new(size),
            pixel_buffer: vec![0; size.width as usize * size.height as usize],
        }
    }

    pub fn add_hittable(&mut self, hittable: HittableObject) {
        self.hittables.hittables.push(hittable);
    }

    pub fn get_size(&self) -> Size<u16> {
        self.accumulated_image.size
    }

    pub fn get_pixel_buffer(&self) -> &[u32] {
        &self.pixel_buffer
    }

    pub fn reset_samples(&mut self) {
        self.samples = 0;
        self.pixel_buffer = vec![0; self.size.width as usize * self.size.height as usize];
    }

    pub fn resize(&mut self, size: Size<u16>) {
        self.size = size;
        self.accumulated_image = Image::new(size);
        self.samples = 0;
        self.pixel_buffer = vec![0; size.width as usize * size.height as usize];
        self.camera = Camera::new(self.camera.eye, size.width as f32 / size.height as f32);
    }

    pub fn render_fragment(&self, uv: &vec2) -> Color {
        let mut ray = self.camera.get_ray(uv);
        let mut color = Color::splat(1.0);
        for depth in 0..MAX_DEPTH {
            if depth == MAX_DEPTH {
                return Color::splat(0.0);
            }
            let hit = self.hittables.hit(&ray, 0.001, f32::INFINITY);

            if let Some(mut hit) = hit {
                if hit.normal.dot(ray.direction) > 0.0 {
                    hit.normal = -hit.normal;
                }
                if let Some(scatter) = hit.material.scatter(&ray, &hit) {
                    color *= scatter.attenuation;
                }

                ray = Ray::new(
                    ray.at(hit.t),
                    hit.normal + vec3::random_in_unit_sphere().normalize(),
                );
            } else {
                let t = (ray.direction.y + 1.0) / 2.0;
                color *= (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
                break;
            }
        }

        color
    }

    pub fn render_sample(&mut self) -> &Vec<u32> {
        let per_pixel = |(i, color): (_, &Color)| {
            let x = (i % self.accumulated_image.size.width as usize) as f32 + rand::random::<f32>();
            let y = (i / self.accumulated_image.size.width as usize) as f32 + rand::random::<f32>();
            let uv = vec2::new(
                x as f32 / (self.size.width - 1) as f32,
                1.0 - (y as f32 / (self.size.height - 1) as f32), // flip
            );
            let fragment = self.render_fragment(&uv);
            if self.samples == 0 {
                fragment
            } else {
                *color + fragment
            }
        };

        #[cfg(feature = "multithread")]
        {
            self.accumulated_image.buffer = self
                .accumulated_image
                .buffer
                .par_iter()
                .enumerate()
                .map(per_pixel)
                .collect();
        }

        #[cfg(not(feature = "multithread"))]
        {
            self.accumulated_image.buffer = self
                .accumulated_image
                .buffer
                .iter()
                .enumerate()
                .map(per_pixel)
                .collect();
        }

        self.samples += 1;

        self.pixel_buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                let out_color =
                    gamma_correct(self.accumulated_image.buffer[i] / self.samples as f32, 2.0);
                let red = ((out_color.r).clamp(0.0, 0.999) * 256.0) as u32;
                let green = ((out_color.g).clamp(0.0, 0.999) * 256.0) as u32;
                let blue = ((out_color.b).clamp(0.0, 0.999) * 256.0) as u32;

                *color = blue | (green << 8) | (red << 16) | (0xFF << 24)
            });

        &self.pixel_buffer
    }

    pub fn render(&mut self) -> &Vec<u32> {
        self.render_sample()
    }
}
