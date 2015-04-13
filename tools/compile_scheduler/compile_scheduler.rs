extern crate getopts;
extern crate common;

use getopts::Options;
use std::env;
use std::process::Command;

use common::hierarchy;
use common::system;
use common::settings;

/// Compiles the scheduler by auto-linking all schedules
fn main() {

    // Program args
    let mut is_child_tool: bool = false;

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("c", "child", "Run as a child compilation tool: i.e. Don't recompile dependent modules and don't modify the .is_compiling file.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("c") {
        is_child_tool = true
    };

    // Lets compile!
    if !is_child_tool {
        hierarchy::set_is_compiling(true).unwrap();
    }

    let mut current_dir = env::current_exe().unwrap(); current_dir.pop();
    let current_dir_name = current_dir.file_name().unwrap();
    let scheduler_filename = current_dir_name.to_os_string().into_string().unwrap() + ".rs";
    let target_dir = current_dir.join("target");

    hierarchy::create_fresh_dir(&target_dir).unwrap();


    let mut command = Command::new(hierarchy::get_rustc_path().as_os_str().to_str().unwrap());

    // Link dependencies dirs
    for path in hierarchy::get_state_dependency_dirs().iter() {
        system::link_libraries(&mut command, path);
    }

    // Link state
    system::link_libraries(&mut command, &hierarchy::get_state_target_dir());

    // Link schedule target dirs
    for schedule_target_dir in hierarchy::get_all_schedule_target_dirs().iter() {
        system::link_libraries(&mut command, schedule_target_dir);
    }

    // Link process target dirs
    for process_target_dir in hierarchy::get_all_process_target_dirs().iter() {
        system::link_libraries(&mut command, process_target_dir);
    }


    let config_display = system::get_compile_config(&mut command, &current_dir, &scheduler_filename, &target_dir);
    command.arg("--crate-type=".to_string() + settings::get_process_lib_type());

    println!("Compiling scheduler {}", config_display);

    system::execute_command(&mut command);

    if !is_child_tool {
        hierarchy::set_is_compiling(false).unwrap();
    }
}
