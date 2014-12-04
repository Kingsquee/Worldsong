use std::os;
use std::io;
use std::io::fs;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the process in the current directory
fn main() {
    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let current_process_filename = current_dir_name.to_string() + ".rs";
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("Compiling {} process", current_dir_name);

    let output = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap())
        .arg("-L").arg(compile_settings::get_common_dir().join("target").as_str().unwrap())
        .arg("-L").arg(compile_settings::get_common_dir().join("target/deps").as_str().unwrap())
        .arg("-L").arg(compile_settings::get_common_dir().join("target/native").as_str().unwrap())
        .arg("--out-dir").arg("./target")
        .arg("--crate-type=".to_string() + compile_settings::get_process_lib_type())
        .arg(current_process_filename)
        .output();

    // Try to run this thing
    let result = match output {
        Ok(r) => r,
        Err(e) => panic!("Failed to run rustc: {}", e),
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
