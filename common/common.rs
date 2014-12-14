#![feature(macro_rules)] 
extern crate sdl2;

pub mod macros;
pub mod data;

#[no_mangle]
pub fn new() -> data::Data {
    data::Data::new()
}
