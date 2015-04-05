#![feature(path_ext)]

extern crate common;

use std::env;
use std::fs::PathExt;
use std::process;
use std::ffi::OsString;

use common::hierarchy;
use common::system;

fn main() {
    let kernel_target_dir = hierarchy::get_kernel_target_dir();
    let kernel_bin = kernel_target_dir.clone().join("kernel");

    let mut command = process::Command::new(kernel_bin);
    command.current_dir(&kernel_target_dir);

    let mut ld_library_paths = OsString::new();

    let key = "LD_LIBRARY_PATH";

    // current ld_library_paths
    match env::var_os(key) {
        Some(val)   => {
            ld_library_paths.push(&val);
            ld_library_paths.push(":");
        }
        None => {
            println!("{} is not defined in the common.", key)
        }
    };

    // common target dir. JUST IN CASE.
    let common_target_dir = hierarchy::get_common_target_dir();
    if !common_target_dir.exists() { panic!("{} doesn't exist. Exiting.") }
    ld_library_paths.push(common_target_dir.as_os_str());
    ld_library_paths.push(":");

    // shared dependencies
    for dir in hierarchy::get_state_dependency_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push(dir.as_os_str());
        ld_library_paths.push(":");
    }

    // state library
    let state_target_dir = hierarchy::get_state_target_dir();
    ld_library_paths.push(state_target_dir.as_os_str());
    ld_library_paths.push(":");

    // all process target dirs
    for dir in hierarchy::get_all_process_target_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push(dir.as_os_str());
        ld_library_paths.push(":");
    }

    // all schedule target dirs
    for dir in hierarchy::get_all_schedule_target_dirs().iter() {
        if !dir.exists() { panic!("{} doesn't exist. Exiting.") }
        ld_library_paths.push(dir.as_os_str());
        ld_library_paths.push(":");
    }

    // scheduler dir
    let scheduler_target_dir = hierarchy::get_scheduler_target_dir();
    ld_library_paths.push(scheduler_target_dir.as_os_str());

    println!("{}{:?}", key, ld_library_paths);

    command.env(key, ld_library_paths);

    system::execute_command(&mut command);
}
