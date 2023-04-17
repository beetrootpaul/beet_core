use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use beet_core::BeetCore;

// TODO: Makefile and format

// TODO: wasm target (with working logging)

const WINDOW_TITLE: &str = "Beet Core: \"Minimal\" examples";
const CANVAS_WIDTH: u32 = 64;
const CANVAS_HEIGHT: u32 = 32;
const CANVAS_SCALE: u32 = 8;
const WINDOW_SIZE: LogicalSize<u32> = LogicalSize::new(CANVAS_WIDTH * CANVAS_SCALE, CANVAS_HEIGHT * CANVAS_SCALE) ;

fn main() {
    // TODO: change this
    BeetCore::hello();

    // TODO: move to lib
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(WINDOW_TITLE)
        .with_inner_size(WINDOW_SIZE)
        .with_min_inner_size(WINDOW_SIZE)
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(CANVAS_WIDTH, CANVAS_HEIGHT, surface_texture).expect("should create a new `pixels` instance")
    };

    // TODO: draw something with use of pixels crate
    event_loop.run(move |event, _, control_flow| {
        // TODO: log instead of printing
        println!("loop iter: event: {:?}", event);

        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // TODO: draw here on pixels.frame_mut()
                pixels.render().unwrap_or_else(|err| {
                    // TODO: log instead of printing
                    println!("failed to render");
                    *control_flow = ControlFlow::Exit;
                });
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}
