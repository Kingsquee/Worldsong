extern crate state;
extern crate sdl2;

use state::{SimulationState, GraphicsState};
use sdl2::pixels::Color;

pub fn execute(sim: &mut SimulationState, window: &GraphicsState) -> () {
    let mut drawer = window.renderer.drawer();

    // While the kernel is running, try modifying these values,
    // compiling this process via the local ./compile, and
    // re-focusing the Worldsong window.

    sim.color_r = sim.color_r.wrapping_add(1);
    sim.color_g = sim.color_g.wrapping_add(1);
    sim.color_b = sim.color_b.wrapping_add(1);

//     println!("{}{}{}", sim.color_r, sim.color_g, sim.color_b);

    // Your changes will be visible immediately. (:
    drawer.set_draw_color(Color::RGB(sim.color_r, sim.color_g, sim.color_b));
    drawer.clear();
    drawer.present();
}