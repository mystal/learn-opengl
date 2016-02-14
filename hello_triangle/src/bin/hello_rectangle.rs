extern crate clap;
#[macro_use]
extern crate glium;
extern crate glutin;

use clap::App;
use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};

#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    let args = App::new("hello_rectangle")
                       .args_from_usage(
                           "-w, --wireframe 'Draw just the wireframe'")
                       .get_matches();

    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    let shapes = [
        Vertex { position: [0.5, 0.5, 0.0] },
        Vertex { position: [0.5, -0.5, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0] },
    ];

    let indices = [
        0u8, 1, 3,
        1, 2, 3,
    ];

    let vertex_buffer = glium::VertexBuffer::new(&window, &shapes).unwrap();
    let index_buffer = glium::IndexBuffer::new(&window, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

    let vertex_shader_src = r#"
        #version 330 core

        layout (location = 0) in vec3 position;

        void main() {
            gl_Position = vec4(position.x, position.y, position.z, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core

        out vec4 color;

        void main() {
            color = vec4(1.0f, 0.5f, 0.2f, 1.0f);
        }
    "#;

    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    let polygon_mode = if args.is_present("wireframe") {
        glium::draw_parameters::PolygonMode::Line
    } else {
        glium::draw_parameters::PolygonMode::Fill
    };

    let draw_params = glium::DrawParameters {
        polygon_mode: polygon_mode,
        .. Default::default()
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
        target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms, &draw_params).unwrap();
        target.finish().unwrap();
    }
}
