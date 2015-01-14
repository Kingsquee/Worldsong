extern crate environment;

use std::os;
use std::io;

use environment::hierarchy;
use environment::system;
use environment::settings;

/// Compiles the process in the current directory
fn main() {
    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let filename = current_dir_name.to_string() + "_macro.rs";
    let target_path = current_dir.join("target");

    hierarchy::create_fresh_dir(&target_path);

    println!("Compiling {} macro", current_dir_name);

    let mut command = io::Command::new(hierarchy::get_rustc_path().as_str().unwrap());
    command.cwd(&current_dir);

    command.arg("--out-dir").arg(target_path.as_str().unwrap());
    command.arg("--crate-type=".to_string() + settings::get_macro_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(filename);

    system::execute_command(&mut command);
}
