extern crate cgmath;
extern crate clap;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;


use std::time::Instant;

use clap::App;
use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};


#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 3],
    texCoord: [f32; 2],
}

implement_vertex!(Vertex, position, texCoord);


fn main() {
    // TODO: Implement passing radius to camera
    let args = App::new("spinning_camera")
                       .args_from_usage(
                           "-r, --radius 'The camera's spin radius'")
                       .get_matches();

    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    // Load vertices
    let shape = &[
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [0.5, -0.5, -0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, 0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [-0.5, 0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [-0.5, 0.5, 0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, 0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [-0.5, -0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [0.5, -0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, -0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, -0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, -0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, -0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [-0.5, 0.5, -0.5], texCoord: [0.0, 1.0] },
        Vertex { position: [0.5, 0.5, -0.5], texCoord: [1.0, 1.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [0.5, 0.5, 0.5], texCoord: [1.0, 0.0] },
        Vertex { position: [-0.5, 0.5, 0.5], texCoord: [0.0, 0.0] },
        Vertex { position: [-0.5, 0.5, -0.5], texCoord: [0.0, 1.0] },
    ];
    let vertex_buffer = glium::VertexBuffer::new(&window, shape).unwrap();
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let cube_positions = &[
        cgmath::vec3( 0.0,  0.0,  0.0f32),
        cgmath::vec3( 2.0,  5.0, -15.0),
        cgmath::vec3(-1.5, -2.2, -2.5),
        cgmath::vec3(-3.8, -2.0, -12.3),
        cgmath::vec3( 2.4, -0.4, -3.5),
        cgmath::vec3(-1.7,  3.0, -7.5),
        cgmath::vec3( 1.3, -2.0, -2.5),
        cgmath::vec3( 1.5,  2.0, -2.5),
        cgmath::vec3( 1.5,  0.2, -1.5),
        cgmath::vec3(-1.3,  1.0, -1.5),
    ];

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
    let vertex_shader_src = include_str!("shaders/spinning_camera.vs");
    let fragment_shader_src = include_str!("shaders/spinning_camera.fs");
    let program = glium::Program::from_source(&window, vertex_shader_src, fragment_shader_src, None).unwrap();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    // Get the time to pass to shader
    let start_time = Instant::now();
    let elapsed_as_f32 = move || {
        let elapsed = start_time.elapsed();
        elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32) / 1_000_000_000.0
    };

    let camera_radius = 10.0;

    loop {
        for event in window.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape)) => return,
                _ => {},
            }
        }

        let (screen_width, screen_height) = window.get_framebuffer_dimensions();

        let elapsed = elapsed_as_f32();
        let cam_x = elapsed.sin() * camera_radius;
        let cam_z = elapsed.cos() * camera_radius;
        let view = cgmath::Matrix4::look_at(cgmath::Point3::new(cam_x, 0.0, cam_z),
                                            cgmath::Point3::new(0.0, 0.0, 0.0),
                                            cgmath::vec3(0.0, 1.0, 0.0));
        let fovy = cgmath::deg(45.0);
        let aspect = screen_width as f32 / screen_height as f32;
        let projection = cgmath::perspective(fovy, aspect, 0.1, 100.0);

        let mut target = window.draw();
        target.clear_color_and_depth((0.2, 0.3, 0.3, 1.0), 1.0);

        for (i, pos) in cube_positions.iter().enumerate() {
            // Create the transformation matrices
            let translate = cgmath::Matrix4::from_translation(pos.clone());
            let rotate_axis = cgmath::vec3(1.0f32, 0.3, 0.5);
            let rotate_angle = cgmath::deg(20.0 * i as f32);
            let rotate: cgmath::Matrix4<f32> = cgmath::Matrix3::from_axis_angle(rotate_axis, rotate_angle.into()).into();
            let model = translate * rotate;

            // Uniforms
            let uniforms = uniform! {
                ourTexture1: &texture1,
                ourTexture2: &texture2,
                model: cgmath::conv::array4x4(model),
                view: cgmath::conv::array4x4(view),
                projection: cgmath::conv::array4x4(projection),
            };

            target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &draw_params).unwrap();
        }

        target.finish().unwrap();
    }
}
