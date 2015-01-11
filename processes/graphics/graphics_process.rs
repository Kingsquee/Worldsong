extern crate state;
//extern crate sdl2;

use state::{SimulationState, GraphicsState};
//use sdl2::pixels::Color;

pub fn execute(sim: &mut SimulationState, window: &GraphicsState) -> () {
    //let renderer = &window.renderer;
    
    println!("HIHIHI");

    // While the kernel is running, try modifying these values,
    // compiling this process via the local ./compile, and
    // re-focusing the Worldsong window.
    sim.color_r -= 1;
    sim.color_g -= 1;
    sim.color_b -= 1;
    /*
    // Your changes will be visible immediately. :)
    let _ = renderer.set_draw_color(Color::RGB(sim.color_r, sim.color_g, sim.color_b));
    let _ = renderer.clear();
    renderer.present(); */
}
