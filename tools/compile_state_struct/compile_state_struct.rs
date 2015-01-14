extern crate environment;

use environment::hierarchy;
use environment::system;

// Compiles the struct in the current directory
fn main() {
    // Regenerate the state
    system::run(&hierarchy::get_state_src_dir().join("generate"), None);
    // Compile the state
    system::run(&hierarchy::get_state_src_dir().join("compile"), None);
}
