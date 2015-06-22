extern crate glium;
extern crate state;
use state::GraphicsState;
use glium::Surface;

// Executes every 1/60th of a second, as defined in the scheduler
pub fn execute(gs: &mut GraphicsState) -> () {
    let mut target = gs.display.draw();

    // Change these values while the application is running,
    // and push the reload key defined in input_process.rs
    gs.color_r += 0.0003f32;
    gs.color_g += 0.002f32;
    gs.color_b += 0.01f32;

    // Sin waves remapped from -1, 1 to 0, 1
    let r = (gs.color_r.sin() + 1f32) / 2f32;
    let g = (gs.color_g.sin() + 1f32) / 2f32;
    let b = (gs.color_b.sin() + 1f32) / 2f32;

    println!("gs.r: {} | sin r: {}", gs.color_r, r);
    println!("gs.g: {} | sin g: {}", gs.color_g, g);
    println!("gs.b: {} | sin b: {}", gs.color_b, b);
    println!("-----------------------------------");

    target.clear_color(gs.color_r.sin(), gs.color_g.sin(), gs.color_b.sin(), 1.0);
    target.finish().unwrap();
}