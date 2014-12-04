use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the schedule by auto-linking all processes
fn main() {
    let current_dir = os::self_exe_path().unwrap();

    let mut schedule_dir = current_dir.clone();
    schedule_dir.pop();

    let schedule_dir_name = schedule_dir.filename_str().unwrap();

    let schedule_filename = schedule_dir_name.to_string() + ".rs";
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    let mut processes_root_dir = current_dir;
    processes_root_dir.pop();
    processes_root_dir.push("processes");

    let mut processes_paths: Vec<&Path> = Vec::new();

    //for each in schedules_root_dir
    let contents = fs::readdir(&processes_root_dir).unwrap();
    for entry in contents.iter() {
        if entry.is_dir() {
            processes_paths.push(entry);
        }
    }

    // debug
    for entry in processes_paths.iter() {
        println!("Process found: {}", entry.display());
    }

    println!("Compiling schedule");

    let mut command = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/deps").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/native").as_str().unwrap());

    for entry in processes_paths.iter() {
        command.arg("-L");
        command.arg(entry.join("target").as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + compile_settings::get_schedules_lib_type());
    command.arg(schedule_filename);

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
