extern crate glium;
use super::types::{Vertex, Color, VertexShader, FragmentShader};
use self::glium::{DisplayBuild, VertexBuffer, IndexBuffer, Program};
use self::glium::index::PrimitiveType;
use self::glium::backend::glutin_backend::GlutinFacade;
use self::glium::glutin::WindowBuilder;

data! (
    GraphicsState {
        display: GlutinFacade = WindowBuilder::new().build_glium().unwrap()

        clear_color: Color = Color { r: 1f32, g: 1f32, b: 1f32, a: 1f32 }

        vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::new(&display,
            vec![
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 1.0, 0.0] },
            ]
        )
        index_buffer: IndexBuffer<u16> = IndexBuffer::new(&display, PrimitiveType::TrianglesList, vec![0u16, 1, 2])

        program: Program = Program::from_source(&display, &VertexShader::load("default"), &FragmentShader::load("default"), None).unwrap()

        reload_shaders: bool = false
    }
);
