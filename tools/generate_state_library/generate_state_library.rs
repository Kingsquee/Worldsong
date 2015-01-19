extern crate "rustc-serialize" as rustc_serialize;
extern crate toml;
extern crate common;

use common::hierarchy;
use common::system;
use common::settings;

mod generate_cargo_toml;
mod generate_source;

// Generates the state library source file
fn main() {
    let struct_src_dirs: Vec<Path> = hierarchy::get_all_struct_src_dirs();
    
    // Don't set is_compilng, since it's just generating the code to compile, not actually compiling it yet.
    generate_cargo_toml::exec(&struct_src_dirs);
    generate_source::exec(&struct_src_dirs);
}
