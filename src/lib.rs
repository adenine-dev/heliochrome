#![feature(portable_simd)]
#![feature(const_trait_impl)]
#![feature(stmt_expr_attributes)]

use std::time::Instant;

use softbuffer::GraphicsContext;
use winit::{
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod heliochrome;
use heliochrome::*;

mod make_context;
use make_context::make_context;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

async fn run(mut context: context::Context, event_loop: EventLoop<()>, window: Window) {
    let mut softbuffer_context = unsafe { GraphicsContext::new(window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::RedrawRequested(window_id) if window_id == softbuffer_context.window().id() => {
                let start = Instant::now();

                context.render_sample();
                softbuffer_context.set_buffer(
                    context.get_pixel_buffer(),
                    context.get_size().width,
                    context.get_size().height,
                );

                let elapsed = start.elapsed();
                println!(
                    "frame render time: {elapsed:?} ({} sample(s))",
                    context.samples
                );
            }
            Event::MainEventsCleared => {
                if context.samples < 500 {
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
                let mut should_update = true;
                match input.virtual_keycode {
                    Some(VirtualKeyCode::A) => {
                        context.camera.eye -= context.camera.horizontal.normalize();
                    }
                    Some(VirtualKeyCode::D) => {
                        context.camera.eye += context.camera.horizontal.normalize();
                    }
                    Some(VirtualKeyCode::W) => {
                        context.camera.eye += context.camera.vertical.normalize();
                    }
                    Some(VirtualKeyCode::S) => {
                        context.camera.eye -= context.camera.vertical.normalize();
                    }
                    Some(VirtualKeyCode::Q) => {
                        context.camera.eye += context
                            .camera
                            .vertical
                            .cross(context.camera.horizontal)
                            .normalize();
                    }
                    Some(VirtualKeyCode::E) => {
                        context.camera.eye -= context
                            .camera
                            .vertical
                            .cross(context.camera.horizontal)
                            .normalize();
                    }
                    _ => should_update = false,
                }
                if should_update && input.state == ElementState::Released {
                    context.reset_samples();
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
