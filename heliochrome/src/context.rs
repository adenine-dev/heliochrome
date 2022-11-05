use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, RwLock};

use indicatif;
use indicatif::ParallelProgressIterator;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuilder};
use spmc;

use crate::color::Color;
use crate::hittables::Hittable;
use crate::image::Image;
use crate::materials::{ScatterType, Scatterable};
use crate::maths::*;
use crate::pdf::{ObjectListPdf, ProbabilityDensityFn};
use crate::scene::Scene;
use crate::tonemap::ToneMap;

#[derive(Clone, Copy)]
pub struct QualitySettings {
    pub samples: u16,
    pub bounces: u16,
}

impl Default for QualitySettings {
    fn default() -> Self {
        Self {
            samples: 250,
            bounces: 12,
        }
    }
}

const RENDER_STATE_INACTIVE: u8 = 0;
const RENDER_STATE_ACTIVE: u8 = 1;
const RENDER_STATE_PAUSED: u8 = 2;
const RENDER_STATE_EXIT: u8 = 3;

pub struct Context {
    pub scene: Arc<RwLock<Scene>>,
    pub quality: QualitySettings,
    size: vec2,
    pub samples: u16,
    accumulated_image: Image,
    out_image: Image,

    pub tone_map: ToneMap,

    // render things
    pixel_sender: Sender<([u8; 4], vec2)>,
    pub pixel_receiver: Receiver<([u8; 4], vec2)>,
    render_state: Arc<AtomicU8>,
    thread_pool: ThreadPool,
    active_threads: Arc<AtomicU32>,
}

const CHUNK_SIZE: usize = 16;

fn gamma_correct(color: Color, gamma: f32) -> Color {
    color.powf(1.0 / gamma)
}

pub fn render_fragment(scene: Arc<RwLock<Scene>>, uv: &vec2, bounces: u16) -> Color {
    let scene = scene.read().unwrap();
    let mut ray = scene.camera.get_ray(uv);
    let mut color = Color::splat(1.0);
    for depth in 0..bounces {
        if depth == bounces {
            return Color::splat(0.0);
        }

        if let Some((intersection, object)) = scene.intersect(&ray, 0.001, f32::INFINITY) {
            let bounce = object.get_bounce_info(&ray, intersection);

            let emitted = object.material.emitted(&bounce);
            if let Some(scatter) = object.material.scatter(&ray, &bounce) {
                match scatter.scatter_type {
                    ScatterType::Pdf(pdf) => {
                        let importants = scene.get_importants();

                        let (scattered, pdf_val) = if importants.is_empty() {
                            let dir = pdf.generate();
                            let scattered = Ray::new(bounce.p, dir);
                            let pdf_val = pdf.value(&scattered.direction);
                            (scattered, pdf_val)
                        } else {
                            let importance_pdf = ObjectListPdf::new(importants, bounce.p);
                            let pdf = (importance_pdf, pdf);

                            let dir = pdf.generate();
                            let scattered = Ray::new(bounce.p, dir);
                            let pdf_val = pdf.value(&scattered.direction);
                            (scattered, pdf_val)
                        };

                        color *= object.material.pdf(&ray, &scattered, &bounce)
                            * scatter.attenuation
                            / pdf_val;
                        color += emitted;
                        ray = scattered;
                    }
                    ScatterType::Specular(specular) => {
                        color *= scatter.attenuation;
                        ray = specular;
                    }
                }
            } else {
                color *= emitted;
                break;
            }
        } else {
            color *= scene.skybox.sample(ray.direction);
            break;
        }
    }

    color.un_nan()
}

impl Context {
    pub fn new(size: vec2, scene: Scene, tone_map: ToneMap) -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        let (pixel_sender, pixel_receiver) = mpsc::channel::<([u8; 4], vec2)>();

        Self {
            scene: Arc::new(RwLock::new(scene)),
            quality: QualitySettings::default(),
            size,
            samples: 0,
            accumulated_image: Image::new(size),
            out_image: Image::new(size),
            tone_map,
            thread_pool,
            pixel_sender,
            pixel_receiver,
            render_state: Arc::new(AtomicU8::new(RENDER_STATE_INACTIVE)),
            active_threads: Arc::new(AtomicU32::new(0)),
        }
    }

    pub fn get_size(&self) -> vec2 {
        self.accumulated_image.size
    }

    pub fn reset_samples(&mut self) {
        self.samples = 0;
        self.accumulated_image = Image::new(self.size);
        self.out_image = Image::new(self.size);
    }

    pub fn resize(&mut self, size: vec2) {
        self.size = size;
        self.accumulated_image = Image::new(size);
        self.out_image = Image::new(size);
        self.samples = 0;
        self.scene.write().unwrap().camera.aspect_ratio = size.x / size.y;
    }

    pub fn render_sample(&mut self) -> Image {
        let mut out_image = Image::new(self.size);
        self.accumulated_image.buffer = self
            .accumulated_image
            .buffer
            .par_iter()
            .progress_count(self.accumulated_image.buffer.len() as u64)
            .enumerate()
            .map(|(i, color): (usize, &Color)| {
                let u = (i % self.size.x as usize) as f32 + rand::random::<f32>();
                let v = (i / self.size.x as usize) as f32 + rand::random::<f32>();
                let uv = vec2::new(
                    u / (self.size.x - 1.0),
                    1.0 - (v / (self.size.y - 1.0)), // flip
                );
                let mut fragment = render_fragment(self.scene.clone(), &uv, self.quality.bounces);
                if self.samples != 0 {
                    fragment += color;
                }
                fragment
            })
            .collect();

        self.samples += 1;
        out_image
            .buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, color)| {
                *color = self.accumulated_image.buffer[i] / self.samples as f32;
            });
        self.tone_map.map(&mut out_image);

        out_image
    }

    pub fn stop_full_render(&mut self) {
        self.render_state
            .store(RENDER_STATE_EXIT, Ordering::Relaxed);
        while self.active_threads.load(Ordering::Relaxed) != 0 {}
        for _ in self.pixel_receiver.try_iter().take(usize::MAX) {}
    }

    pub fn toggle_pause_full_render(&self) {
        let rs = self.render_state.load(Ordering::Acquire);
        if rs == RENDER_STATE_ACTIVE {
            self.render_state
                .store(RENDER_STATE_PAUSED, Ordering::Release);
        } else if rs == RENDER_STATE_PAUSED {
            self.render_state
                .store(RENDER_STATE_ACTIVE, Ordering::Release);
        }
    }

    pub fn start_full_render(&self) {
        self.render_state
            .store(RENDER_STATE_ACTIVE, Ordering::Relaxed);
        self.active_threads.store(0, Ordering::Relaxed);

        let (mut rs, rr) = spmc::channel::<(usize, usize)>();

        let cx = (self.size.x / CHUNK_SIZE as f32).ceil() as i32;
        let cy = (self.size.y / CHUNK_SIZE as f32).ceil() as i32;

        let mut tx = 0;
        let mut ty = 0;
        let mut dx = 0;
        let mut dy = -1;
        for _ in 0..((cx + 1).max(cy + 1).pow(2)) {
            if (-cx / 2 <= tx) && (tx <= cx / 2) && (-cy / 2 <= ty) && (ty <= cy / 2) {
                rs.send(((tx + (cx / 2)) as usize, (ty + (cy / 2)) as usize))
                    .unwrap();
            }

            if (tx == ty) || ((tx < 0) && (tx == -ty)) || ((tx > 0) && (tx == 1 - ty)) {
                (dx, dy) = (-dy, dx)
            }
            tx += dx;
            ty += dy;
        }

        let thread_count = 7;

        for _ in 0..thread_count {
            let dtx = self.pixel_sender.clone();

            let render_state = self.render_state.clone();
            let active_threads = self.active_threads.clone();
            let rrs = rr.clone();
            let size = self.size;
            let scene = self.scene.clone();
            let quality = self.quality;
            self.thread_pool.spawn(move || {
                active_threads.fetch_add(1, Ordering::Acquire);
                while let Ok((tx, ty)) = rrs.recv() {
                    'y: for y in 0..CHUNK_SIZE {
                        'x: for x in 0..CHUNK_SIZE {
                            let x = x + ((tx) as usize * CHUNK_SIZE);
                            let y = y + ((ty) as usize * CHUNK_SIZE);
                            if x >= size.x as usize {
                                continue 'x;
                            }
                            if y >= size.y as usize {
                                continue 'y;
                            }

                            let i = x + (size.x as usize * y);
                            let mut c = Color::splat(0.0);
                            for _ in 0..quality.samples {
                                if render_state.load(Ordering::Acquire) == RENDER_STATE_PAUSED {
                                    while render_state.load(Ordering::Acquire)
                                        == RENDER_STATE_PAUSED
                                    {
                                        std::thread::yield_now();
                                    }
                                    std::thread::yield_now();
                                }
                                if render_state.load(Ordering::Acquire) == RENDER_STATE_EXIT {
                                    active_threads.fetch_sub(1, Ordering::Acquire);
                                    return;
                                }

                                let u = (i % size.x as usize) as f32 + rand::random::<f32>();
                                let v = (i / size.x as usize) as f32 + rand::random::<f32>();
                                let uv = vec2::new(
                                    u / (size.x - 1.0),
                                    1.0 - (v / (size.y - 1.0)), // flip
                                );
                                let fragment = render_fragment(scene.clone(), &uv, quality.bounces);
                                c += fragment;
                            }

                            let out_color = gamma_correct(c / quality.samples as f32, 2.0);
                            let red = ((out_color.r).clamp(0.0, 0.999) * 256.0) as u8;
                            let green = ((out_color.g).clamp(0.0, 0.999) * 256.0) as u8;
                            let blue = ((out_color.b).clamp(0.0, 0.999) * 256.0) as u8;

                            dtx.send((
                                [red, green, blue, 0xff],
                                // blue | (green << 8) | (red << 16) | (0xFF << 24),
                                vec2::new(x as f32, y as f32),
                            ))
                            .unwrap();
                        }
                    }
                }
                active_threads.fetch_sub(1, Ordering::Acquire);
            });
        }

        drop(rs);
    }
}
