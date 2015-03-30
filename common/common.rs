#![feature(os)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(old_fs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
mod internal;

pub mod hierarchy;
pub mod system;
pub mod settings;
pub mod macros;
