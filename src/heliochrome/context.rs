use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::maths::*;

#[cfg(feature = "multithread")]
use rayon::prelude::*;

use super::hittables::{Hit, Hittable};
use super::object::Object;

pub struct Context {
    pub camera: Camera,
    pub skybox: Option<Image>,

    objects: Vec<Object>,

    size: vec2,
    pub samples: u16,
    accumulated_image: Image,

    pixel_buffer: Vec<u32>,
}

const MAX_DEPTH: u32 = 50;

fn gamma_correct(color: Color, gamma: f32) -> Color {
    color.powf(1.0 / gamma)
}

impl Context {
    pub fn new(size: vec2, camera: Camera) -> Self {
        Self {
            camera,
            skybox: None,
            objects: vec![],
            size,
            samples: 0,
            accumulated_image: Image::new(size),
            pixel_buffer: vec![0; size.x as usize * size.y as usize],
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_size(&self) -> vec2 {
        self.accumulated_image.size
    }

    pub fn get_pixel_buffer(&self) -> &[u32] {
        &self.pixel_buffer
    }

    pub fn reset_samples(&mut self) {
        self.samples = 0;
        self.pixel_buffer = vec![0; self.size.x as usize * self.size.y as usize];
        self.accumulated_image = Image::new(self.size);
    }

    pub fn resize(&mut self, size: vec2) {
        self.size = size;
        self.accumulated_image = Image::new(size);
        self.samples = 0;
        self.pixel_buffer = vec![0; size.x as usize * size.y as usize];
        self.camera.aspect_ratio = size.x / size.y;
    }

    pub fn render_fragment(&self, uv: &vec2) -> Color {
        let mut ray = self.camera.get_ray(uv);
        let mut color = Color::splat(1.0);
        for depth in 0..MAX_DEPTH {
            if depth == MAX_DEPTH {
                return Color::splat(0.0);
            }

            let (hit, object) = {
                let mut t_max = f32::INFINITY;
                let t_min = 0.001;

                let mut hit: Option<Hit> = None;
                let mut obj = None;
                for object in self.objects.iter() {
                    let new_hit = object.hit(&ray, t_min, t_max);
                    if new_hit.is_some() {
                        hit = new_hit;
                        obj = Some(object);
                        t_max = hit.as_ref().unwrap().t;
                    }
                }

                (hit, obj)
            };

            if let Some(hit) = hit {
                // normals
                // let n = 0.5 * (hit.normal.normalized() + vec3::splat(1.0));
                // color = Color::new(n.x, n.y, n.z);
                // break;
                if let Some(scatter) = object.unwrap().get_scatter(&ray, &hit) {
                    color *= scatter.attenuation;
                    ray = scatter.outgoing;
                }
            } else {
                if let Some(skybox) = &self.skybox {
                    let uv = vec2::new(
                        0.5 + ray.direction.z.atan2(ray.direction.x) / std::f32::consts::TAU,
                        0.5 + ray.direction.y.asin() / std::f32::consts::PI,
                    );
                    color *= skybox.sample_uv(&uv);
                } else {
                    let t = (ray.direction.y + 1.0) / 2.0;
                    color *= (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
                }
                break;
            }
        }

        color
    }

    pub fn render_sample(&mut self) -> &Vec<u32> {
        let per_pixel = |(i, color): (_, &Color)| {
            let x = (i % self.accumulated_image.size.x as usize) as f32 + rand::random::<f32>();
            let y = (i / self.accumulated_image.size.x as usize) as f32 + rand::random::<f32>();
            let uv = vec2::new(
                x as f32 / (self.size.x - 1.0),
                1.0 - (y as f32 / (self.size.y - 1.0)), // flip
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
}
