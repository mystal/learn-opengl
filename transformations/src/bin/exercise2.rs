extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;


use std::time::Instant;

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
    let vertex_shader_src = include_str!("shaders/exercise2.vs");
    let fragment_shader_src = include_str!("shaders/exercise2.fs");
    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Get the time to pass to shader
    let start_time = Instant::now();
    let elapsed_as_f32 = move || {
        let elapsed = start_time.elapsed();
        elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32) / 1_000_000_000.0
    };

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }

        // Create the transformation matrices
        let translate = cgmath::Matrix4::from_translation(cgmath::vec3(0.5f32, -0.5, 0.0));
        let rotate_axis = cgmath::vec3(0.0f32, 0.0, 1.0);
        let rotate_angle = cgmath::deg(elapsed_as_f32() * 50.0);
        let rotate: cgmath::Matrix4<f32> = cgmath::Matrix3::from_axis_angle(rotate_axis, rotate_angle.into()).into();
        let transform1 = translate * rotate;

        let translate = cgmath::Matrix4::from_translation(cgmath::vec3(-0.5f32, 0.5, 0.0));
        let scale = cgmath::Matrix4::from_scale(elapsed_as_f32().sin());
        let transform2 = translate * scale;

        // Uniforms
        let uniforms1 = uniform! {
            ourTexture1: &texture1,
            ourTexture2: &texture2,
            transform: cgmath::conv::array4x4(transform1),
        };
        let uniforms2 = uniform! {
            ourTexture1: &texture1,
            ourTexture2: &texture2,
            transform: cgmath::conv::array4x4(transform2),
        };

        let mut target = window.draw();
        target.clear_color(0.2, 0.3, 0.3, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms1, &Default::default()).unwrap();
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms2, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
