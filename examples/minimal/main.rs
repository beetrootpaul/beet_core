use beet_core::BeetCore;

fn main() {
    // TODO: change this
    BeetCore::hello();

    // TODO: move to lib
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();

    // TODO: draw something with use of pixels crate
    event_loop.run(move |event, _, control_flow| {
        // TODO: log instead of printing
        println!("loop iter: event: {:?}", event);

        *control_flow = winit::event_loop::ControlFlow::Wait;

        match event {
            winit::event::Event::WindowEvent {
                event: winit::event::WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = winit::event_loop::ControlFlow::Exit,
            _ => (),
        }
    });
}