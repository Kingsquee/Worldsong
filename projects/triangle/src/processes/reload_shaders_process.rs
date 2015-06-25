extern crate state;
extern crate glium;
use std::thread;
use glium::Program;
use state::{GraphicsState};
use state::types::{VertexShader, FragmentShader};

pub fn execute(graphics_state: &mut GraphicsState) -> () {
    if graphics_state.reload_shaders {
        graphics_state.program = reload_shaders(&graphics_state);
        graphics_state.reload_shaders = false;
    }
}

fn reload_shaders(graphics_state: &GraphicsState) -> Program {
    loop {
        match Program::from_source(&graphics_state.display, &VertexShader::load("default"), &FragmentShader::load("default"), None) {
            Err(e) => {
                println!("ERROR: {}", e);
                thread::sleep_ms(1000);
                continue
            }
            Ok(o) => {
                return o
            }
        };
    }
}
