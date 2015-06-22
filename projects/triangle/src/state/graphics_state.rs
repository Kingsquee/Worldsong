extern crate glium;
use super::types::Vertex;
use self::glium::{DisplayBuild, VertexBuffer, IndexBuffer, Program};
use self::glium::index::PrimitiveType;
use self::glium::backend::glutin_backend::GlutinFacade;
use self::glium::glutin::WindowBuilder;

const VERTEX_SHADER_SOURCE: &'static str = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &'static str = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 1.0, 1.0);
    }
"#;

data! (
    GraphicsState {
        display: GlutinFacade = WindowBuilder::new().build_glium().unwrap()

        vertex_buffer: VertexBuffer<Vertex> = VertexBuffer::new(&display,
            vec![
                Vertex { position: [-0.5, -0.5], color: [0.0, 1.0, 0.0] },
                Vertex { position: [ 0.0,  0.5], color: [0.0, 0.0, 1.0] },
                Vertex { position: [ 0.5, -0.5], color: [1.0, 1.0, 0.0] },
            ]
        )
        index_buffer: IndexBuffer<u16> = IndexBuffer::new(&display, PrimitiveType::TrianglesList, vec![0u16, 1, 2])

        program: Program = Program::from_source(&display, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE, None).unwrap()
    }
);