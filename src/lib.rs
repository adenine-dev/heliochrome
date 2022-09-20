#![feature(map_first_last)]
#![feature(portable_simd)]
#![feature(const_trait_impl)]
#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
// lib but not and maths
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use std::{
    error,
    ffi::CString,
    thread,
    time::{SystemTime, UNIX_EPOCH},
};

use indicatif::{ProgressBar, ProgressStyle};
use instant::{self, Duration};
use log::info;
use softbuffer::GraphicsContext;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod load_obj;

mod heliochrome;
use heliochrome::{context::RenderTask, maths::vec2, *};

mod make_context;
use make_context::make_context;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen_rayon::init_thread_pool;

const NUM_SAMPLES: u16 = 500;

fn write_image(size: vec2, buffer: &[u32]) -> Result<(), Box<dyn error::Error>> {
    #[cfg(target_arch = "wasm32")]
    {
        Err("Cannot save file on web, try right clicking the canvas.")?
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?;
        let pathstr = CString::new(format!("out_{}.png", now.as_secs()))?;

        let data = buffer
            .iter()
            .map(|x| {
                let bytes = x.to_be_bytes();
                let r = bytes[1];
                let g = bytes[2];
                let b = bytes[3];
                let a = bytes[0];

                [r, g, b, a]
            })
            .flatten()
            .collect::<Vec<u8>>();

        stb::image_write::stbi_write_png(&pathstr, size.x as i32, size.y as i32, 4, &data, 0);

        Ok(())
    }
}

async fn run(mut context: context::Context, event_loop: EventLoop<()>, window: Window) {
    let mut softbuffer_context = unsafe { GraphicsContext::new(window) }.unwrap();
    // #[cfg(not(target_arch = "wasm32"))]
    // let pb = ProgressBar::new(NUM_SAMPLES as u64);
    // #[cfg(not(target_arch = "wasm32"))]
    // pb.set_style(
    //     ProgressStyle::with_template(
    //         "{spinner:.magenta} {elapsed_precise} {msg:>16} |{wide_bar}| {pos}/{len} samples",
    //     )
    //     .unwrap()
    //     .tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘 ")
    //     .progress_chars(""),
    // );
    let mut size = context.get_size();
    let mut pixel_buffer = vec![0; size.x as usize * size.y as usize];
    let mut render_task = RenderTask::new(context);
    render_task.render();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(window_id) if window_id == softbuffer_context.window().id() => {
                #[cfg(not(target_arch = "wasm32"))]
                let start = instant::Instant::now();
                softbuffer_context.set_buffer(&pixel_buffer, size.x as u16, size.y as u16);

                // #[cfg(not(target_arch = "wasm32"))]
                // {
                //     let elapsed = start.elapsed();
                //     pb.inc(1);
                //     pb.set_message(format!("{elapsed:?}"));
                // }
            }
            Event::MainEventsCleared => {
                render_task
                    .data_receiver
                    .try_iter()
                    .take(100)
                    .for_each(|(c, idx)| {
                        pixel_buffer[idx] = c;
                    });

                softbuffer_context.window().request_redraw();

                // if context.samples < NUM_SAMPLES {
                // } else {
                //     *control_flow = ControlFlow::Wait;
                // }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(newsize),
                window_id,
            } if window_id == softbuffer_context.window().id() => {
                if newsize.width as f32 != size.x || newsize.height as f32 != size.y {
                    render_task.kill();
                    size = vec2::new(newsize.width as f32, newsize.height as f32);
                    render_task
                        .context
                        .write()
                        .unwrap()
                        .resize(maths::vec2::new(
                            newsize.width as f32,
                            newsize.height as f32,
                        ));
                    pixel_buffer = vec![0; newsize.width as usize * newsize.height as usize];
                    render_task.render();
                }

                // #[cfg(not(target_arch = "wasm32"))]
                // pb.reset();
            }
            Event::WindowEvent {
                window_id,
                event:
                    WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                        is_synthetic: _,
                    },
            } if window_id == softbuffer_context.window().id() => {
                // #[allow(deprecated)] // deprecated because this allows behavior to exist on web
                // if input.virtual_keycode == Some(VirtualKeyCode::S) && input.modifiers.ctrl() {
                //     if input.state == ElementState::Released {
                //         if let Err(err) =
                //             write_image(context.get_size(), context.get_pixel_buffer())
                //         {
                //             info!("Could not write image :<\n{}", err.to_string())
                //         } else {
                //             info!("Wrote image.");
                //         }
                //     }
                //     return;
                // }
                // let mut should_update = true;
                // let camera_speed = 0.1;
                // match input.virtual_keycode {
                //     Some(VirtualKeyCode::A) => {
                //         context.camera.eye -= (context.camera.at - context.camera.eye)
                //             .cross(context.camera.up)
                //             .normalize()
                //             * camera_speed;
                //     }
                //     Some(VirtualKeyCode::D) => {
                //         context.camera.eye += (context.camera.at - context.camera.eye)
                //             .cross(context.camera.up)
                //             .normalize()
                //             * camera_speed;
                //     }
                //     Some(VirtualKeyCode::W) => {
                //         context.camera.eye +=
                //             (context.camera.at - context.camera.eye).normalize() * camera_speed;
                //     }
                //     Some(VirtualKeyCode::S) => {
                //         context.camera.eye -=
                //             (context.camera.at - context.camera.eye).normalize() * camera_speed;
                //     }
                //     Some(VirtualKeyCode::Q) => {
                //         context.camera.eye += context.camera.up.normalize() * camera_speed;
                //     }
                //     Some(VirtualKeyCode::E) => {
                //         context.camera.eye -= context.camera.up.normalize() * camera_speed;
                //     }
                //     _ => should_update = false,
                // }
                // if should_update {
                //     context.reset_samples();
                //     #[cfg(not(target_arch = "wasm32"))]
                //     pb.reset();
                //     softbuffer_context.window().request_redraw();
                // }
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == softbuffer_context.window().id() => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }
    });
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn heliochrome() {
    let context = make_context();

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(
            context.get_size().x as u16,
            context.get_size().y as u16,
        ))
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(context, event_loop, window));
    }

    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        use winit::platform::web::WindowExtWebSys;
        // On wasm, append the canvas to the document body
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| {
                body.append_child(&web_sys::Element::from(window.canvas()))
                    .ok()
            })
            .expect("couldn't append canvas to document body");
        wasm_bindgen_futures::spawn_local(run(context, event_loop, window));
    }
}
