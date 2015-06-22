extern crate glium;
use self::glium::DisplayBuild;
use self::glium::backend::glutin_backend::GlutinFacade;
use self::glium::glutin::WindowBuilder;

// Creates a public struct with a constructor
data! (
    GraphicsState {
        display: GlutinFacade = WindowBuilder::new().build_glium().unwrap()
        color_r: f32 = 0f32
        color_g: f32 = 0f32
        color_b: f32 = 0f32
    }
);