#![feature(os)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(old_fs)]
#![feature(core)]
#![feature(collections)]
#![feature(str_char)]

extern crate rustc_serialize;
extern crate toml;
extern crate common;

use common::hierarchy;

use std::old_path::Path;
use std::old_path::GenericPath;

mod generate_cargo_toml;
mod generate_source;

// Generates the state library source file
fn main() {
    let struct_src_dirs: Vec<Path> = hierarchy::get_all_struct_src_dirs();

    // Don't set is_compilng, since it's just generating the code to compile, not actually compiling it yet.
    generate_cargo_toml::exec(&struct_src_dirs);
    generate_source::exec(&struct_src_dirs);
}
