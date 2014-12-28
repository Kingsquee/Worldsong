extern crate getopts;

use getopts::{optopt,optflag,getopts,OptGroup};

use std::os;
use std::io;

#[path = "./../tool_settings.rs"]
mod tool_settings;

#[path = "./../tool_helpers.rs"]
mod tool_helpers;

#[path = "./../../../common/fs.rs"]
mod fs;

/// Compiles the common lib, and everything else, wot.
fn main() {
    // Program args
    let mut should_update: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("u", "update", "Update libraries before compiling.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("u") {
        should_update = true
    };

    // Lets compile!
    fs::set_is_compiling(true);

    let current_dir = os::self_exe_path().unwrap();
    let target_path = current_dir.join("target");

    fs::create_fresh_dir(&target_path);

    println!("Compiling the Common library");

    if should_update {
        let mut update_common_command = io::Command::new(fs::get_cargo_path().as_str().unwrap());
        update_common_command.cwd(&current_dir);
        update_common_command.arg("update");
        tool_helpers::execute_command(&mut update_common_command);
    }

    let mut compile_common_command = io::Command::new(fs::get_cargo_path().as_str().unwrap());
    compile_common_command.cwd(&current_dir);
    compile_common_command.arg("build");
    tool_helpers::execute_command(&mut compile_common_command);


    for path in fs::get_all_process_src_dirs().iter_mut() {
        tool_helpers::run_external_application(&path.join("compile"), Some(vec!["-c"]));
    }

    for path in fs::get_all_schedule_src_dirs().iter_mut() {
        tool_helpers::run_external_application(&path.join("compile"), Some(vec!["-c"]));
    }

    tool_helpers::run_external_application(
        &fs::get_scheduler_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    tool_helpers::run_external_application(
        &fs::get_kernel_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    fs::set_is_compiling(false);

}
