#![feature(os)]
#![feature(old_io)]
#![feature(old_path)]
#![feature(old_fs)]
#![feature(core)]

extern crate common;

use std::os;
use std::old_io;
use std::old_io::fs::PathExtensions;
use std::old_path::Path;
use std::old_path::GenericPath;

use common::hierarchy;
use common::system;
use common::settings;

fn main() {
    let kernel_target_dir = hierarchy::get_kernel_target_dir();
    let kernel_bin = kernel_target_dir.clone().join("kernel");

    let mut command = old_io::Command::new(kernel_bin);
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
            println!("{} is not defined in the common.", key)
        }
    };

    // common target dir. JUST IN CASE.
    let common_target_dir = hierarchy::get_common_target_dir();
    if !common_target_dir.exists() { panic!("{} doesn't exist. Exiting.") }
    ld_library_paths.push_str(common_target_dir.as_str().unwrap());
    ld_library_paths.push_str(":");

    // shared dependencies
    for dir in hierarchy::get_state_dependency_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }

    // state library
    let state_target_dir = hierarchy::get_state_target_dir();
    ld_library_paths.push_str(state_target_dir.as_str().unwrap());
    ld_library_paths.push_str(":");

    // all process target dirs
    for dir in hierarchy::get_all_process_target_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }

    // all schedule target dirs
    for dir in hierarchy::get_all_schedule_target_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push_str(dir.as_str().unwrap());
        ld_library_paths.push_str(":");
    }

    // scheduler dir
    let scheduler_target_dir = hierarchy::get_scheduler_target_dir();
    ld_library_paths.push_str(scheduler_target_dir.as_str().unwrap());

    println!("{}{}", key, ld_library_paths);

    command.env(key, ld_library_paths);

    system::execute_command(&mut command);
}
