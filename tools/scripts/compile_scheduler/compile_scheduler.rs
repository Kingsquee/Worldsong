use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the scheduler by auto-linking all schedules
fn main() {

//     compile_settings::set_system_paths();

    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let scheduler_filename = current_dir_name.to_string() + ".rs";
    let target_path = current_dir.join("target");

    match fs::mkdir(&target_path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    println!("Compiling scheduler");

    let mut command = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/deps").as_str().unwrap());
    command.arg("-L").arg(compile_settings::get_common_dir().join("target/native").as_str().unwrap());

    let mut schedule_dirs: Vec<&Path> = Vec::new();

    let schedules_dir = fs::readdir(&compile_settings::get_schedules_dir()).unwrap();
    for schedule_dir in schedules_dir.iter() {
        if schedule_dir.is_dir() {
            schedule_dirs.push(schedule_dir);
        }
    }

    // debug
    for schedule_dir in schedule_dirs.iter() {
        println!("Schedule found: {}", schedule_dir.filename_str().unwrap());
    }

    for schedule_dir in schedule_dirs.iter() {
        command.arg("-L");
        command.arg(schedule_dir.join("schedule/target").as_str().unwrap());
    }

    let mut process_dirs: Vec<Path> = Vec::new();

    for schedule_dir in schedule_dirs.iter() {
        process_dirs.push_all(compile_settings::get_all_process_dirs(schedule_dir.filename_str().unwrap()).as_slice());
    }

    // debug
    for process_dir in process_dirs.iter() {
        println!("Process found: {}", process_dir.filename_str().unwrap());
    }

    for process_dir in process_dirs.iter() {
        command.arg("-L");
        command.arg(process_dir.join("target").as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + compile_settings::get_scheduler_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(scheduler_filename);

    compile_settings::execute_command(command);
}
