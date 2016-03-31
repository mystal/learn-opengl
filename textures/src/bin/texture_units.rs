#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;

use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
    texCoord: [f32; 2],
}

implement_vertex!(Vertex, position, color, texCoord);

fn main() {
    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    // Load vertices
    let shape = vec![
        Vertex { position: [0.5, 0.5, 0.0], color: [1.0, 0.0, 0.0], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], color: [1.0, 1.0, 0.0], texCoord: [0.0, 1.0] },
    ];
    let indices = [
        0u8, 1, 3,
        1, 2, 3,
    ];
    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();
    let index_buffer = glium::IndexBuffer::new(&window, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    // Load textures
    let image = image::open("assets/container.jpg").unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture1 = glium::texture::Texture2d::new(&window, image).unwrap();

    let image = image::open("assets/awesomeface.png").unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture2 = glium::texture::Texture2d::new(&window, image).unwrap();

    // Load shaders
    let vertex_shader_src = include_str!("shaders/texture_units.vs");
    let fragment_shader_src = include_str!("shaders/texture_units.fs");
    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Uniforms
    let uniforms = uniform! {
        ourTexture1: &texture1,
        ourTexture2: &texture2,
    };

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }

        let mut target = window.draw();
        target.clear_color(0.2, 0.3, 0.3, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
