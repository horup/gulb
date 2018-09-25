// https://docs.rs/glutin/0.18.0/glutin/
// https://docs.rs/winit/0.17.2/winit/
extern crate winit;
use winit::{WindowBuilder, EventsLoop, Event, WindowEvent, ControlFlow};
use winit::dpi::{LogicalSize};

fn main() 
{
    let mut events_loop = EventsLoop::new();
    let _window = WindowBuilder::new()
                .with_title("Hello")
                .with_dimensions(LogicalSize::new(640.0, 480.0))
                .build(&events_loop);

    events_loop.run_forever(|event| 
    {
        match event 
        {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => ControlFlow::Break,
            _ => ControlFlow::Continue,
        }
    });
}