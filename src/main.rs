extern crate glium;
extern crate glutin;

use glium::{DisplayBuild, Surface};
use glutin::{Api, GlRequest};

fn main() {
    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    loop {
        for event in window.poll_events() {
            let mut target = window.draw();
            target.clear_color(0.0, 0.0, 1.0, 1.0);
            target.finish().unwrap();

            match event {
                glutin::Event::Closed => return,
                _ => {},
            }
        }
    }
}
