use std::os;
use std::io;
use std::io::fs;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the common lib, wot.
fn main() {
    let current_dir = os::self_exe_path().unwrap();
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("Compiling the Common library");

    let output = io::Command::new(compile_settings::get_cargo_path().as_str().unwrap())
        .arg("build")
        .output();

    // Try to run this thing
    let result = match output {
        Ok(r) => r,
        Err(e) => panic!("Failed to run cargo: {}", e),
    };

    // If it ran, how'd it do?
    match result.status.success() {
        true => {
            println!("{}", String::from_utf8(result.output).unwrap());
        }
        false => {
            println!("{}", String::from_utf8(result.error).unwrap());
            os::set_exit_status(1)
        }
    };
}
