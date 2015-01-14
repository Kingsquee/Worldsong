extern crate getopts;
extern crate environment;

use getopts::{optopt,optflag,getopts,OptGroup};

use std::os;
use std::io;

use environment::hierarchy;
use environment::system;
use environment::settings;

/// Compiles the state lib, and everything else, wot.
fn main() {
    // Program args
    let mut should_update: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("u", "update", "Update all state structs' dependencies before compiling.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("u") {
        should_update = true
    };

    // Lets compile!
    hierarchy::set_is_compiling(true);

    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let target_path = current_dir.join("target");
    let source_filename = current_dir_name.to_string() + ".rs";

    hierarchy::create_fresh_dir(&target_path);

    println!("Compiling the State library");

    if should_update {
        let mut cargo_update_command = io::Command::new(hierarchy::get_cargo_path().as_str().unwrap());
        cargo_update_command.cwd(&hierarchy::get_state_src_dir());
        cargo_update_command.arg("update");
        system::execute_command(&mut cargo_update_command);
    }

    let mut cargo_build_command = io::Command::new(hierarchy::get_cargo_path().as_str().unwrap());
    cargo_build_command.cwd(&hierarchy::get_state_src_dir());
    cargo_build_command.arg("build");
    system::execute_command(&mut cargo_build_command);

    // Recompile everything
    
    // Recompile processes
    for path in hierarchy::get_all_process_src_dirs().iter_mut() {
        system::run(&path.join("compile"), Some(vec!["-c"]));
    }

    // Recompile schedules
    for path in hierarchy::get_all_schedule_src_dirs().iter_mut() {
        system::run(&path.join("compile"), Some(vec!["-c"]));
    }

    // Recompile the scheduler
    system::run(
        &hierarchy::get_scheduler_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    // Recompile kernel
    system::run(
        &hierarchy::get_kernel_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    hierarchy::set_is_compiling(false);
}
