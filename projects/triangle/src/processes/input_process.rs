extern crate glutin;
extern crate state;
use state::{GraphicsState, CoreState};
use glutin::Event::{Closed, KeyboardInput};
use glutin::ElementState::{Pressed};
use glutin::VirtualKeyCode as VK;

pub fn execute(graphics_state: &mut GraphicsState, core_state: &mut CoreState) -> () {
    for event in graphics_state.display.poll_events() {
        match event {
            Closed => {
                core_state.quit = true
            }
            KeyboardInput(Pressed, _, Some(VK::Escape)) => {
                core_state.quit = true
            }
            KeyboardInput(Pressed, _, Some(VK::F3)) => {
                graphics_state.reload_shaders = true;
            }
            KeyboardInput(Pressed, _, Some(VK::F4)) => {
                core_state.reload = true
            }
            KeyboardInput(Pressed, _, Some(VK::F5)) => {
                core_state.reset = true
            }
            _ => ()
        }
    }
}
