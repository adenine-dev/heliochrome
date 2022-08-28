#![feature(portable_simd)]
#![feature(const_trait_impl)]

use maths::{vec2, vec3};
use softbuffer::GraphicsContext;
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod camera;
mod color;
mod context;
mod image;
mod maths;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

async fn run(event_loop: EventLoop<()>, window: Window) {
    let mut context = context::Context::new(
        maths::Size::new(
            window.inner_size().width as u16,
            window.inner_size().height as u16,
        ),
        camera::Camera::new(
            vec3::new(0.0, 0.0, 0.0),
            window.inner_size().width as f32 / window.inner_size().height as f32,
        ),
    );

    let mut softbuffer_context = unsafe { GraphicsContext::new(window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == softbuffer_context.window().id() => {
                context.render();
                softbuffer_context.set_buffer(
                    context.get_pixel_buffer(),
                    context.get_size().width,
                    context.get_size().height,
                );
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
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(400, 225))
        .build(&event_loop)
        .unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        // Temporarily avoid srgb formats for the swapchain on the web
        pollster::block_on(run(event_loop, window));
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
        wasm_bindgen_futures::spawn_local(run(event_loop, window));
    }
}
