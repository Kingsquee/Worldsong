extern crate common;

use std::os;
use std::io;
use std::io::fs::PathExtensions;

use common::hierarchy;
use common::system;
use common::settings;

fn main() {
    let kernel_target_dir = hierarchy::get_kernel_target_dir();
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

    // common target dir. JUST IN CASE.
    ld_library_paths.push_str(hierarchy::get_common_target_dir().as_str().unwrap());
    ld_library_paths.push_str(":");
    
    // shared dependencies 
    for dir in hierarchy::get_dependencies_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    
    // state library
    ld_library_paths.push_str(hierarchy::get_state_target_dir().as_str().unwrap());
    ld_library_paths.push_str(":");
    
    // all process target dirs
    for dir in hierarchy::get_all_process_target_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    
    // all schedule target dirs
    for dir in hierarchy::get_all_schedule_target_dirs().iter() {
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }
    
    // scheduler dir
    ld_library_paths.push_str(hierarchy::get_scheduler_target_dir().as_str().unwrap());

    println!("{}{}", key, ld_library_paths);

    command.env(key, ld_library_paths);

    system::execute_command(&mut command);
}
