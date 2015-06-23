extern crate glium;
extern crate state;
use state::{GraphicsState};

use glium::Surface;
use glium::uniforms::EmptyUniforms;

pub fn execute(gs: &mut GraphicsState) -> () {
    let mut target = gs.display.draw();

    let clear_color = &mut gs.clear_color;

    // Change these values while the application is running,
    // and push the reload key defined in input_process.rs
    clear_color.r += 0.0003f32;
    clear_color.g += 0.002f32;
    clear_color.b += 0.01f32;

    // Sin waves remapped from -1, 1 to 0, 1
    let r = (clear_color.r.sin() + 1f32) / 2f32;
    let g = (clear_color.g.sin() + 1f32) / 2f32;
    let b = (clear_color.b.sin() + 1f32) / 2f32;

    println!("gs.r: {} | sin r: {}", clear_color.r, r);
    println!("gs.g: {} | sin g: {}", clear_color.g, g);
    println!("gs.b: {} | sin b: {}", clear_color.b, b);
    println!("-----------------------------------");

    target.clear_color(clear_color.r.sin(), clear_color.g.sin(), clear_color.b.sin(), clear_color.a);
    target.draw(&gs.vertex_buffer, &gs.index_buffer, &gs.program, &EmptyUniforms, &Default::default()).unwrap();
    target.finish().unwrap();
}