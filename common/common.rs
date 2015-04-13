#![feature(path_ext)]
#![feature(unicode)]

//extern crate regex;

extern crate unicode;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod internal;

pub mod hierarchy;
pub mod system;
pub mod settings;
pub mod macros;
