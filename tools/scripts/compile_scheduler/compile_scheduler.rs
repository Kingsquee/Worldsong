use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the scheduler by auto-linking all schedules
fn main() {
    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let scheduler_filename = current_dir_name.to_string() + ".rs";
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    let schedules_root_dir = compile_settings::get_schedules_dir();
    let mut schedules_paths: Vec<&Path> = Vec::new();

    //for each in schedules_root_dir
    let contents = fs::readdir(&schedules_root_dir).unwrap();
    for entry in contents.iter() {
        if entry.is_dir() {
            schedules_paths.push(entry);
        }
    }

    // debug
    for entry in schedules_paths.iter() {
        println!("Schedule found: {}", entry.display());
    }

    println!("Compiling scheduler");

    let mut command = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/deps").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/native").as_str().unwrap());

    for entry in schedules_paths.iter() {
        command.arg("-L");
        command.arg(entry.join("schedule/target").as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + compile_settings::get_scheduler_lib_type());
    command.arg(scheduler_filename);

    let output = command.output();

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
