#![crate_name="PlaceholderProcess"]
extern crate dataset;
extern crate debug;
extern crate sdl2;

use dataset::DB;
use sdl2::pixels;
use sdl2::event;
use sdl2::keycode;

/// Called asap. TODO: Change to deltatime
#[no_mangle]
pub fn per_cycle(data: &mut DB) {

    match event::poll_event() {
        event::QuitEvent(_) => data.stop_execution = true,
        event::KeyDownEvent(_, _, key, _, _) => {
            if key == keycode::EscapeKey {
                data.stop_execution = true;
            }
        },
        _ => {}
    }
        
}

/// Called every 1/60th of a second.
#[no_mangle]
pub fn per_frame(data: &mut DB) {
    //println!("{}", data.window.is_some());
    let renderer = data.renderer.as_ref().unwrap();
    
    data.color_r += 1;
    data.color_g -= 1;
    data.color_b *= 2;
    
    let _ = renderer.set_draw_color(pixels::RGB(data.color_r, data.color_g, data.color_b));
    let _ = renderer.clear();
    renderer.present();
    
}