extern crate glutin;
extern crate state;
use state::{GraphicsState, CoreState};

pub fn execute(graphics_state: &GraphicsState, core_state: &mut CoreState) -> () {
    for event in graphics_state.display.poll_events() {
        match event {
            glutin::Event::Closed => {
                core_state.quit = true
            }
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => {
                core_state.quit = true
            }
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Space)) => {
                core_state.reload = true
            }
            glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::F5)) => {
                core_state.reset = true
            }
            _ => ()
        }
    }
}