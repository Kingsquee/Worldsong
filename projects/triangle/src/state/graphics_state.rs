extern crate glium;
use super::types::{Vertex, Color, VertexShader, FragmentShader};
use self::glium::{DisplayBuild, VertexBuffer, IndexBuffer, Program};
use self::glium::index::PrimitiveType;
use self::glium::backend::glutin_backend::GlutinFacade;
use self::glium::glutin::WindowBuilder;

data! (
    GraphicsState {
        display: GlutinFacade = WindowBuilder::new().build_glium().unwrap()

        clear_color: Color = Color { r: 0f32, g: 0f32, b: 0f32, a: 1f32 }

        vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::new(&display,
            &[
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 1.0, 0.0] },
            ]
        ).unwrap()
        index_buffer: IndexBuffer<u16> = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2]).unwrap()

        program: Program = Program::from_source(&display, &VertexShader::load("default"), &FragmentShader::load("default"), None).unwrap()

        reload_shaders: bool = false
    }
);
