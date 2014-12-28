#![feature(macro_rules)] 
extern crate sdl2;

pub mod macros;
pub mod state;
pub mod fs;

#[no_mangle]
pub fn new() -> state::Data {
    state::Data::new()
}
