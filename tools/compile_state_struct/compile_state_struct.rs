#![feature(os)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(old_fs)]

extern crate common;

use common::hierarchy;
use common::system;
use std::old_path::Path;
use std::old_path::GenericPath;

// Compiles the struct in the current directory
fn main() {
    // Regenerate the state
    system::run(&hierarchy::get_state_src_dir().join("generate"), None);
    // Compile the state
    system::run(&hierarchy::get_state_src_dir().join("compile"), None);
}
