extern crate glium;
extern crate state;
use state::{GraphicsState};
use glium::Surface;
use glium::uniforms::EmptyUniforms;

pub fn execute(gs: &mut GraphicsState) -> () {
    let mut target = gs.display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&gs.vertex_buffer, &gs.index_buffer, &gs.program, &EmptyUniforms, &Default::default()).unwrap();
    target.finish().unwrap();
}