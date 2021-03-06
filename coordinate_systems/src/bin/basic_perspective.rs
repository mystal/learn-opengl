extern crate cgmath;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;


use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};


#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    texCoord: [f32; 2],
}

implement_vertex!(Vertex, position, texCoord);


fn main() {
    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    // Load vertices
    let shape = vec![
        Vertex { position: [0.5, 0.5, 0.0], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.0], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.0], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.0], texCoord: [0.0, 1.0] },
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
    let vertex_shader_src = include_str!("shaders/basic_perspective.vs");
    let fragment_shader_src = include_str!("shaders/basic_perspective.fs");
    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }


        let (screen_width, screen_height) = window.get_framebuffer_dimensions();

        // Set up model, view, and projection matrices
        let rotate_axis = cgmath::vec3(1.0f32, 0.0, 0.0);
        let rotate_angle = cgmath::deg(-55.0);
        let rotate: cgmath::Matrix4<f32> = cgmath::Matrix3::from_axis_angle(rotate_axis, rotate_angle.into()).into();
        let model = rotate;
        let view = cgmath::Matrix4::from_translation(cgmath::vec3(0.0f32, 0.0, -3.0));
        let fovy = cgmath::deg(45.0);
        let aspect = screen_width as f32 / screen_height as f32;
        let projection = cgmath::perspective(fovy, aspect, 0.1, 100.0);

        // Uniforms
        let uniforms = uniform! {
            ourTexture1: &texture1,
            ourTexture2: &texture2,
            model: cgmath::conv::array4x4(model),
            view: cgmath::conv::array4x4(view),
            projection: cgmath::conv::array4x4(projection),
        };

        let mut target = window.draw();
        target.clear_color(0.2, 0.3, 0.3, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();
    }
}
