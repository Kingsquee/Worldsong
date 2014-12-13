extern crate common;
extern crate sdl2;

use common::data::Data;
use sdl2::event;
use sdl2::event::Event;
use sdl2::keycode;

pub fn execute(data: &mut Data) -> () {
    match event::poll_event() {
        Event::Quit(_) => data.scheduler.quit = true,
        Event::KeyDown(_, _, key, _, _, _) => {
            if key == keycode::KeyCode::Escape {
                data.scheduler.quit = true;
            }
        },
        Event::Window(_, _, id, _, _) => {
            if id as int == event::WindowEventId::FocusGained as int {
                if data.window.first_focus {
                    data.window.first_focus = false;
                    return;
                }
                if !data.scheduler.reload {
                    data.scheduler.reload = true;
                }
            }
        }
        _ => {}
    }
}
