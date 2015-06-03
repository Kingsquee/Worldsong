extern crate state;

use state::{SimulationState};

pub fn execute(sim: &mut SimulationState) -> () {
    println!("{}|{}|{}", sim.color_r, sim.color_g, sim.color_b);
}