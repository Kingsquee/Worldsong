#![feature(std_misc)]
extern crate worldsong_hierarchy;
extern crate system;

use std::path::{PathBuf};
use std::process;
use std::dynamic_lib::DynamicLibrary;


// TODO: Make this work differently if statically compiled?
//extern crate worldsong_config;
// TODO: enable reloading when error code are returned.

fn main() {
    let app_dir = worldsong_hierarchy::get_current_project_dir();

    //let kernel_target_dir = worldsong_hierarchy::get_module_target_dir(&app_dir, "kernel");
    let kernel_bin = worldsong_hierarchy::get_module_target_bin(&app_dir, "kernel");

    let mut command = process::Command::new(kernel_bin);
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

    println!("{}{:?}", path_envvar, formatted_path);

    command.env(path_envvar, formatted_path);

    system::execute_command(&mut command);
}
