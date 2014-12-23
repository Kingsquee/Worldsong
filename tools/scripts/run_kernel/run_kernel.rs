use std::os;
use std::io;
use std::io::fs::PathExtensions;

#[path = "./../tool_settings.rs"]
mod tool_settings;

#[path = "./../tool_helpers.rs"]
mod tool_helpers;

#[path = "./../../../common/fs.rs"]
mod fs;

fn main() {

    let kernel_target_dir = fs::get_kernel_target_dir();
    let kernel_bin = kernel_target_dir.clone().join("kernel");

    let mut command = io::Command::new(kernel_bin);
    command.cwd(&kernel_target_dir);

    let mut ld_library_paths = String::new();

    let key = "LD_LIBRARY_PATH";

    // current ld_library_paths
    let current_ld_library_paths = match os::getenv(key) {
        Some(val)   => {
            ld_library_paths.push_str(val.as_slice());
            ld_library_paths.push_str(":");
        }
        None => {
            println!("{} is not defined in the environment.", key)
        }
    };

    // common target dirs
    for dir in fs::get_all_common_target_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    // all process target dirs
    for dir in fs::get_all_process_target_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    // all schedule target dirs
    for dir in fs::get_all_schedule_target_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    // scheduler dir
    ld_library_paths.push_str(fs::get_scheduler_target_dir().as_str().unwrap());

    println!("{}{}", key, ld_library_paths);

    command.env(key, ld_library_paths);

    tool_helpers::execute_command(&mut command);
}
