use cgmath::{self, Matrix3, Matrix4, Vector2, Vector3};
use cgmath::prelude::*;
use glium::{self, Surface};


const VERTEX_SHADER_SRC: &'static str = include_str!("shaders/sprite.vs");
const FRAGMENT_SHADER_SRC: &'static str = include_str!("shaders/sprite.fs");

const QUAD_VERTICES: &'static [Vertex] = &[
    Vertex { vertex: [0.0, 1.0, 0.0, 1.0] },
    Vertex { vertex: [1.0, 0.0, 1.0, 0.0] },
    Vertex { vertex: [0.0, 0.0, 0.0, 0.0] },
    Vertex { vertex: [0.0, 1.0, 0.0, 1.0] },
    Vertex { vertex: [1.0, 1.0, 1.0, 1.0] },
    Vertex { vertex: [1.0, 0.0, 1.0, 0.0] },
];


#[derive(Clone, Copy)]
struct Vertex {
    vertex: [f32; 4],
}

implement_vertex!(Vertex, vertex);


pub struct SpriteRenderer {
    shader: glium::Program,
    vertex_buffer: glium::VertexBuffer<Vertex>,
}

impl SpriteRenderer {
    pub fn new(display: &glium::Display) -> Self {
        let shader = glium::Program::from_source(display, VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC, None).unwrap();
        let vertex_buffer = glium::VertexBuffer::new(display, QUAD_VERTICES).unwrap();

        SpriteRenderer {
            shader: shader,
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn with_shader(display: &glium::Display, shader: glium::Program) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(display, QUAD_VERTICES).unwrap();

        SpriteRenderer {
            shader: shader,
            vertex_buffer: vertex_buffer,
        }
    }

    pub fn draw_sprite(&self, texture: &glium::Texture2d, projection: Matrix4<f32>,
                       position: Vector2<f32>, size: Vector2<f32>, rotate: f32,
                       color: Vector3<f32>, target: &mut glium::Frame) {
        let model = {
            let translate = Matrix4::from_translation(position.extend(0.0));
            let rotate_axis = cgmath::vec3(0.0f32, 0.0, 1.0);
            let rotate_angle = cgmath::deg(rotate);
            let rotate_rotation: Matrix4<f32> = Matrix3::from_axis_angle(rotate_axis, rotate_angle.into()).into();
            let rotate =
                Matrix4::from_translation(cgmath::vec3(0.5 * size.x, 0.5 * size.y, 0.0)) *
                rotate_rotation *
                Matrix4::from_translation(cgmath::vec3(-0.5 * size.x, -0.5 * size.y, 0.0));
            let scale = Matrix4::from_nonuniform_scale(size.x, size.y, 1.0);
            translate * rotate * scale
        };

        let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        let uniforms = uniform! {
            image: texture,
            spriteColor: cgmath::conv::array3(color),
            model: cgmath::conv::array4x4(model),
            view: cgmath::conv::array4x4(Matrix4::<f32>::identity()),
            projection: cgmath::conv::array4x4(projection),
        };

        target.draw(&self.vertex_buffer, &index_buffer, &self.shader, &uniforms, &Default::default()).unwrap();
    }
}
