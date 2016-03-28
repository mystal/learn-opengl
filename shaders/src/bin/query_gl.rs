extern crate clap;
#[macro_use]
extern crate glium;
extern crate glutin;

use clap::App;
use glium::{CapabilitiesSource, DisplayBuild};
use glutin::{Api, GlRequest};

fn main() {
    let args = App::new("query_gl")
                       .get_matches();

    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    println!("Capabilities: {:#?}", window.get_capabilities());
}
