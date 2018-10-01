// https://docs.rs/glutin/0.18.0/glutin/
// https://docs.rs/winit/0.17.2/winit/
// http://www.opengl-tutorial.org/beginners-tutorials/tutorial-2-the-first-triangle/

extern crate winit;
extern crate glutin;
extern crate gl;
extern crate cgmath;
use winit::{WindowBuilder, EventsLoop, Event, WindowEvent};
use winit::dpi::{LogicalSize};
use glutin::{GlWindow, ContextBuilder, GlContext};
use std::time::{Instant, Duration};

use gl::*;

mod primitive;
mod render;
use self::render::{Render};
use self::primitive::*;

fn main() 
{
    let mut events_loop = EventsLoop::new();
    let window_builder = WindowBuilder::new()
                .with_title("Gulb!")
                .with_dimensions(LogicalSize::new(512.0, 512.0));

    let context_builder = ContextBuilder::new().with_vsync(true);
    let gl_window = GlWindow::new(window_builder, context_builder, &events_loop).unwrap();
    
    let mut render = Render::new();

    let mut prim = Primitive::new();
    prim.scale = 0.01;
    
    render.set_primitive(0, prim);
    unsafe
    {
        gl_window.make_current().unwrap();
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
       // gl::Enable(POLYGON_SMOOTH);
      //  gl::Enable(BLEND);

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
            let primitive = render.get_primitive(0);
            primitive.scale += 0.0001;
            primitive.translate.x += 0.0001;
            render.draw();
        }

        gl_window.swap_buffers().unwrap();

        let diff:Duration = Instant::now() - now;
        let ms = diff.as_secs() as f64 + diff.subsec_micros() as f64 / 1000.0;
        println!("{}", ms);
    }
}