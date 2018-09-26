// https://docs.rs/glutin/0.18.0/glutin/
// https://docs.rs/winit/0.17.2/winit/
extern crate winit;
extern crate glutin;
extern crate gl;
use winit::{WindowBuilder, EventsLoop, Event, WindowEvent};
use winit::dpi::{LogicalSize};
use glutin::{GlWindow, ContextBuilder, GlContext};
use std::time::{Instant, Duration};

mod render;
use self::render::{Render};

fn main() 
{
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
                .with_title("Gulb!")
                .with_dimensions(LogicalSize::new(640.0, 480.0));

    let context_builder = ContextBuilder::new().with_vsync(true);
    let gl_window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
    
    let mut render = Render::new();

    unsafe
    {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);

        render.init();
    }

    let mut running = true;
    while running
    {
        let now = Instant::now();
        events_loop.poll_events(|event| 
        {
            match event
            {
                Event::WindowEvent{ event, .. } => match event
                {
                    WindowEvent::CloseRequested => running = false,
                    _ => ()
                },
                _ => ()
            }
        });

        unsafe 
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            render.draw();
        }

        gl_window.swap_buffers().unwrap();

        let diff:Duration = Instant::now() - now;
        let ms = diff.as_secs() as f64 + diff.subsec_micros() as f64 / 1000.0;
     //   println!("{}", ms);
    }
}