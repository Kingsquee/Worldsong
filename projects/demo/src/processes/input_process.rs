extern crate state;
extern crate sdl2;

use state::{CoreState, GraphicsState};
use sdl2::event;
use sdl2::event::Event;
use sdl2::keycode::KeyCode;

pub fn execute(core: &mut CoreState, window: &mut GraphicsState) -> () {
    let mut event_pump = window.sdl.event_pump();
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit{..} => core.quit = true,
            Event::KeyDown{ keycode: KeyCode::Escape, ..} => {
                core.quit = true;
            },
            Event::KeyDown{ keycode: KeyCode::F5, ..} => {
                core.reset = true;
            },
            Event::Window{ win_event_id, ..} => {
                if win_event_id as isize == event::WindowEventId::FocusGained as isize {
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
}
