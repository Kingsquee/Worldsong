extern crate common;

use common::hierarchy;
use common::system;

// Compiles the struct in the current directory
fn main() {
    // Regenerate the state
    system::run_external_application(&hierarchy::get_state_src_dir().join("generate"), None);
    // Compile the state
    system::run_external_application(&hierarchy::get_state_src_dir().join("compile"), None);
}
