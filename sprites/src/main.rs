extern crate cgmath;
extern crate clap;
#[macro_use]
extern crate glium;
extern crate glutin;
extern crate image;

use std::collections::HashSet;
use std::time::{Duration, Instant};

use cgmath::prelude::*;
use clap::App;
use glium::{DisplayBuild, Surface};
use glutin::{Api, ElementState, Event, GlRequest, VirtualKeyCode};

mod sprite;


fn duration_as_f64(d: Duration) -> f64 {
    d.as_secs() as f64 + (d.subsec_nanos() as f64 / 1_000_000_000.0)
}


fn main() {
    // Build a window with OpenGL 3.3
    let window = glutin::WindowBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_glium()
        .unwrap();

    //let win_ref = window.get_window().expect("Could not get Glutin window!");
    //win_ref.set_cursor_state(glutin::CursorState::Hide);

    // Load textures
    let image = image::open("assets/awesomeface.png").unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    println!("Image dimensions: {:?}", image_dimensions);
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
    let texture = glium::Texture2d::new(&window, image).unwrap();

    let draw_params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let sprite_renderer = sprite::SpriteRenderer::new(&window);

    // Get the time to pass to shader
    let start_time = Instant::now();
    let mut last_frame_time = start_time;
    let elapsed_as_f32 = move || {
        let elapsed = start_time.elapsed();
        elapsed.as_secs() as f32 + (elapsed.subsec_nanos() as f32) / 1_000_000_000.0
    };

    // let mut camera_pos = cgmath::vec3(0.0f32, 0.0, 3.0);
    // let mut camera_front = cgmath::vec3(0.0f32, 0.0, -1.0);
    // let mut camera_up = cgmath::vec3(0.0f32, 1.0, 0.0);

    let mut keys = HashSet::new();

    let (screen_width, screen_height) = window.get_framebuffer_dimensions();

    loop {
        let this_frame_time = Instant::now();
        let dt = duration_as_f64(this_frame_time - last_frame_time);
        last_frame_time = this_frame_time;

        for event in window.poll_events() {
            match event {
                Event::Closed => return,
                Event::KeyboardInput(ElementState::Pressed, _, Some(keycode)) => {
                    keys.insert(keycode);
                },
                Event::KeyboardInput(ElementState::Released, _, Some(keycode)) => {
                    keys.remove(&keycode);
                },
                Event::MouseMoved((x, y)) => {
                },
                _ => {},
            }
        }

        if keys.contains(&VirtualKeyCode::Escape) {
            break;
        }
        if keys.contains(&VirtualKeyCode::W) {
        }
        if keys.contains(&VirtualKeyCode::S) {
        }
        if keys.contains(&VirtualKeyCode::D) {
        }
        if keys.contains(&VirtualKeyCode::A) {
        }

        let (screen_width, screen_height) = window.get_framebuffer_dimensions();
        //println!("({}, {})", screen_width, screen_height);

        let elapsed = elapsed_as_f32();
        //let view = cgmath::Matrix4::<f32>::identity();
        let projection = cgmath::ortho(0.0, screen_width as f32, 0.0, screen_height as f32, -1.0, 1.0);

        let mut target = window.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        sprite_renderer.draw_sprite(&texture, projection, cgmath::vec2(200.0, 200.0),
                                    cgmath::vec2(512.0, 512.0), 0.0, cgmath::vec3(0.0, 1.0, 0.0),
                                    &mut target);

        target.finish().unwrap();
    }
}
