extern crate state;
extern crate sdl2;

use state::{KernelState, GraphicsState};
use sdl2::event;
use sdl2::event::Event;
use sdl2::keycode;

pub fn execute(kernel: &mut KernelState, window: &mut GraphicsState) -> () {
    match event::poll_event() {
        Event::Quit(_) => kernel.quit = true,
        Event::KeyDown(_, _, key, _, _, _) => {
            if key == keycode::KeyCode::Escape {
                kernel.quit = true;
            }
        },
        Event::Window(_, _, id, _, _) => {
            if id as isize == event::WindowEventId::FocusGained as isize {
                if window.first_focus {
                    window.first_focus = false;
                    return;
                }
                if !kernel.reload {
                    kernel.reload = true;
                }
            }
        }
        _ => {}
    }
}
