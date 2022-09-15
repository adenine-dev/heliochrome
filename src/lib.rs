#![feature(portable_simd)]
#![feature(const_trait_impl)]
#![feature(stmt_expr_attributes)]
#![allow(dead_code)]
#![feature(let_chains)]

use std::{
    error,
    ffi::CString,
    time::{SystemTime, UNIX_EPOCH},
};

use indicatif::{ProgressBar, ProgressStyle};
use instant;
use log::info;
use softbuffer::GraphicsContext;
use stb;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod load_obj;

mod heliochrome;
use heliochrome::{camera::Camera, maths::Size, *};

mod make_context;
use make_context::make_context;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const NUM_SAMPLES: u16 = 500;

fn write_image(size: Size<u16>, buffer: &[u32]) -> Result<(), Box<dyn error::Error>> {
    #[cfg(target_arch = "wasm32")]
    {
        Err("Cannot save file on web, try right clicking the canvas.")?;
    }

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

    stb::image_write::stbi_write_png(&pathstr, size.width as i32, size.height as i32, 4, &data, 0);

    Ok(())
}

async fn run(mut context: context::Context, event_loop: EventLoop<()>, window: Window) {
    let mut softbuffer_context = unsafe { GraphicsContext::new(window) }.unwrap();
    #[cfg(not(target_arch = "wasm32"))]
    let pb = ProgressBar::new(NUM_SAMPLES as u64);
    #[cfg(not(target_arch = "wasm32"))]
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.magenta} {elapsed_precise} {msg:>16} |{wide_bar}| {pos}/{len} samples",
        )
        .unwrap()
        .tick_chars("🌑🌒🌓🌔🌕🌖🌗🌘 ")
        .progress_chars(""),
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(window_id) if window_id == softbuffer_context.window().id() => {
                #[cfg(not(target_arch = "wasm32"))]
                let start = instant::Instant::now();

                context.render_sample();
                softbuffer_context.set_buffer(
                    context.get_pixel_buffer(),
                    context.get_size().width,
                    context.get_size().height,
                );

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let elapsed = start.elapsed();
                    pb.inc(1);
                    pb.set_message(format!("{elapsed:?}"));
                }

                // println!(
                //     "frame render time: {elapsed:?} ({} sample(s))",
                //     context.samples
                // );
            }
            Event::MainEventsCleared => {
                if context.samples < NUM_SAMPLES {
                    softbuffer_context.window().request_redraw();
                } else {
                    *control_flow = ControlFlow::Wait;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id,
            } if window_id == softbuffer_context.window().id() => {
                context.resize(maths::Size {
                    width: size.width as u16,
                    height: size.height as u16,
                });
                #[cfg(not(target_arch = "wasm32"))]
                pb.reset();
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
                //
                if input.virtual_keycode == Some(VirtualKeyCode::S) && input.modifiers.ctrl() {
                    if input.state == ElementState::Released {
                        if let Err(err) =
                            write_image(context.get_size(), context.get_pixel_buffer())
                        {
                            info!("Could not write image :<\n{}", err.to_string())
                        } else {
                            info!("Wrote image.");
                        }
                    }
                    return;
                }
                let mut should_update = true;
                let camera_speed = 0.1;
                match input.virtual_keycode {
                    Some(VirtualKeyCode::A) => {
                        context.camera.eye -= (context.camera.at - context.camera.eye)
                            .cross(context.camera.up)
                            .normalize()
                            * camera_speed;
                    }
                    Some(VirtualKeyCode::D) => {
                        context.camera.eye += (context.camera.at - context.camera.eye)
                            .cross(context.camera.up)
                            .normalize()
                            * camera_speed;
                    }
                    Some(VirtualKeyCode::W) => {
                        context.camera.eye +=
                            (context.camera.at - context.camera.eye).normalize() * camera_speed;
                    }
                    Some(VirtualKeyCode::S) => {
                        context.camera.eye -=
                            (context.camera.at - context.camera.eye).normalize() * camera_speed;
                    }
                    Some(VirtualKeyCode::Q) => {
                        context.camera.eye += context.camera.up.normalize() * camera_speed;
                    }
                    Some(VirtualKeyCode::E) => {
                        context.camera.eye -= context.camera.up.normalize() * camera_speed;
                    }
                    _ => should_update = false,
                }
                if should_update {
                    context.reset_samples();
                    #[cfg(not(target_arch = "wasm32"))]
                    pb.reset();
                    softbuffer_context.window().request_redraw();
                }
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub fn heliochrome() {
    let context = make_context();

    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(
            context.get_size().width,
            context.get_size().height,
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
