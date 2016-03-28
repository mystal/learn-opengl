#[macro_use]
extern crate glium;
extern crate glutin;

use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    let shape = vec![
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0] },
        Vertex { position: [0.0, 0.5, 0.0] },
    ];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 330 core

        layout (location = 0) in vec3 position;

        out vec4 vertexColor;

        void main() {
            gl_Position = vec4(position, 1.0);
            vertexColor = vec4(0.5, 0.0, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core

        in vec4 vertexColor;

        out vec4 color;

        void main() {
            color = vertexColor;
        }
    "#;

    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

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
        target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
