extern crate common;
extern crate sdl2;

use common::data::{CoreState, WinState};
use sdl2::event;
use sdl2::event::Event;
use sdl2::keycode;

pub fn execute(core: &mut CoreState, window: &mut WinState) -> () {
    match event::poll_event() {
        Event::Quit(_) => core.quit = true,
        Event::KeyDown(_, _, key, _, _, _) => {
            if key == keycode::KeyCode::Escape {
                core.quit = true;
            }
        },
        Event::Window(_, _, id, _, _) => {
            if id as int == event::WindowEventId::FocusGained as int {
                if window.first_focus {
                    window.first_focus = false;
                    return;
                }
                if !core.reload {
                    core.reload = true;
                }
            }
        }
        _ => {}
    }
}
