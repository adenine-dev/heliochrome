use std::collections::{HashSet, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex, RwLock};
use std::thread::{self, Thread};

use spmc;
// use threadpool::ThreadPool;

use crate::camera::Camera;
use crate::color::Color;
use crate::image::Image;
use crate::maths::*;

#[cfg(feature = "multithread")]
use rayon::prelude::*;

use rayon::{ThreadPool, ThreadPoolBuilder};

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
const CHUNK_SIZE: usize = 16;

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
                self.accumulated_image.buffer[i] + fragment
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

pub struct RenderTask {
    data_sender: Sender<([u8; 4], vec2)>,
    pub data_receiver: Receiver<([u8; 4], vec2)>,
    should_exit: Arc<AtomicBool>,
    thread_pool: ThreadPool,
    active_threads: Arc<AtomicU32>,
    pub context: Arc<RwLock<Context>>,
}

impl RenderTask {
    pub fn new(context: Arc<RwLock<Context>>) -> Self {
        let thread_pool = ThreadPoolBuilder::new().build().unwrap();
        // let thread_pool = ThreadPool::new(
        //     (thread::available_parallelism()
        //         .unwrap_or(std::num::NonZeroUsize::new(1).unwrap())
        //         .get() as i32
        //         - 2)
        //     .max(1) as usize,
        // );

        let (data_sender, data_receiver) = mpsc::channel::<([u8; 4], vec2)>();

        RenderTask {
            data_sender,
            data_receiver,
            should_exit: Arc::new(AtomicBool::new(false)),
            active_threads: Arc::new(AtomicU32::new(0)),
            thread_pool,
            context,
        }
    }

    pub fn kill(&mut self) {
        self.should_exit.store(true, Ordering::Relaxed);

        // has the effect of killing all active threads by dropping the previous thread pool, not great but yknow
        while self.active_threads.load(Ordering::Relaxed) != 0 {}
        // self.thread_pool = ThreadPoolBuilder::new().build().unwrap();
        for _ in self.data_receiver.try_iter().take(usize::MAX) {}
    }

    pub fn render(&self) {
        self.should_exit.store(false, Ordering::Relaxed);
        self.active_threads.store(0, Ordering::Relaxed);

        let (mut rs, rr) = spmc::channel::<(usize, usize)>();

        let cx = (self.context.read().unwrap().size.x / CHUNK_SIZE as f32).ceil() as i32;
        let cy = (self.context.read().unwrap().size.y / CHUNK_SIZE as f32).ceil() as i32;
        let chunks = cx * cy;

        let mut tx: i32 = 0;
        let mut ty: i32 = 0;
        let mut dx = 0;
        let mut dy = -1;
        for _ in 0..(cx.max(cy).pow(2)) {
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

        let thread_count = 6;

        for i in 0..thread_count {
            let dtx = self.data_sender.clone();
            let ctx = self.context.clone();
            let should_exit = self.should_exit.clone();
            let active_threads = self.active_threads.clone();
            let rrs = rr.clone();
            self.thread_pool.spawn(move || {
                active_threads.fetch_add(1, Ordering::Acquire);
                while let Ok((tx, ty)) = rrs.recv() {
                    let context = ctx.read().unwrap();
                    'y: for y in 0..CHUNK_SIZE {
                        'x: for x in 0..CHUNK_SIZE {
                            let x = x + ((tx) as usize * CHUNK_SIZE);
                            let y = y + ((ty) as usize * CHUNK_SIZE);
                            if x >= context.size.x as usize {
                                continue 'x;
                            }
                            if y >= context.size.y as usize {
                                continue 'y;
                            }

                            let i = x + (context.size.x as usize * y);
                            let mut c = Color::splat(0.0);
                            let sample_count = 50;
                            for _ in 0..sample_count {
                                if should_exit.load(Ordering::Relaxed) {
                                    active_threads.fetch_sub(1, Ordering::Acquire);
                                    return;
                                }
                                let u = (i % context.accumulated_image.size.x as usize) as f32
                                    + rand::random::<f32>();
                                let v = (i / context.accumulated_image.size.x as usize) as f32
                                    + rand::random::<f32>();
                                let uv = vec2::new(
                                    u / (context.size.x - 1.0),
                                    1.0 - (v / (context.size.y - 1.0)), // flip
                                );
                                let fragment = context.render_fragment(&uv);
                                c += fragment;
                            }

                            let out_color = gamma_correct(c / sample_count as f32, 2.0);
                            let red = ((out_color.r).clamp(0.0, 0.999) * 256.0) as u8;
                            let green = ((out_color.g).clamp(0.0, 0.999) * 256.0) as u8;
                            let blue = ((out_color.b).clamp(0.0, 0.999) * 256.0) as u8;

                            dtx.send((
                                [red, green, blue, 0xff],
                                // blue | (green << 8) | (red << 16) | (0xFF << 24),
                                vec2::new(x as f32, y as f32),
                            ));
                        }
                    }
                }
                active_threads.fetch_sub(1, Ordering::Acquire);
                // println!("end {i}");
            });
        }

        drop(rs);
    }
}
