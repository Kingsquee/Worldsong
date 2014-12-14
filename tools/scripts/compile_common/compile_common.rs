use std::os;
use std::io;
use std::io::fs;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the common lib, and everything else, wot.
fn main() {
    let current_dir = os::self_exe_path().unwrap();
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("Compiling the Common library");

    let mut compile_common_command = io::Command::new(compile_settings::get_cargo_path().as_str().unwrap());
    compile_common_command.arg("build");

    compile_settings::execute_command(compile_common_command);

    //TODO: cwd = appropriate dir. compile processes, sequences, sequencer, kernel, in that order.
}
