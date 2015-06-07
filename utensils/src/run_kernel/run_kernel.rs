#![feature(std_misc)]
extern crate worldsong_hierarchy;
extern crate system;

use std::path::{PathBuf};
use std::process;
use std::dynamic_lib::DynamicLibrary;
use std::process::{Command, Stdio};

const RELOAD_STATE_STATUS_CODE: i32 = 3;

fn main() {

    loop {
        let status_code = run_kernel();
        if status_code != RELOAD_STATE_STATUS_CODE {
            break
        }
    }

}

fn run_kernel() -> i32 {
    let app_dir = worldsong_hierarchy::get_current_project_dir();

    //let kernel_target_dir = worldsong_hierarchy::get_module_target_dir(&app_dir, "kernel");
    let kernel_bin = worldsong_hierarchy::get_module_target_bin(&app_dir, "kernel");

    let mut command = Command::new(kernel_bin);
    //command.current_dir(&kernel_target_dir);

    let mut library_dirs: Vec<PathBuf> = Vec::new();

    let path_envvar = DynamicLibrary::envvar();

    let current_search_path = DynamicLibrary::search_path();

    library_dirs.push(worldsong_hierarchy::get_module_target_dir(&app_dir, "processes"));
    library_dirs.push(worldsong_hierarchy::get_module_target_dir(&app_dir, "schedules"));
    library_dirs.push(worldsong_hierarchy::get_module_target_dir(&app_dir, "scheduler"));
    library_dirs.push(worldsong_hierarchy::get_module_target_dir(&app_dir, "state"));
    library_dirs.push(worldsong_hierarchy::get_dependencies_deps_target_dir(&app_dir));
    library_dirs.push(worldsong_hierarchy::get_dependencies_native_target_dir(&app_dir));

    for entry in current_search_path.iter() {
        library_dirs.push(entry.clone());
    }

    let formatted_path = DynamicLibrary::create_path(&library_dirs);

    //println!("{}{:?}", path_envvar, formatted_path);

    command.env(path_envvar, formatted_path);

    // Try to run this thing
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());

    let status = match command.status() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    match status.code() {
        Some(code) => code,
        None => 1 //On Unix, this will return None if the process was terminated by a signal.
    }
}