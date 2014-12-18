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

/// Compiles the process in the current directory
fn main() {

    // Program args
    let mut is_child_script: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("c", "child", "Run as a child compilation script: i.e. Don't recompile dependent modules and don't modify the .iscompiling file.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("c") {
        is_child_script = true
    };

    // Lets compile!
    if !is_child_script {
        fs::set_is_compiling(true);
    }

    let current_dir = os::self_exe_path().unwrap();
    let current_dir_name = current_dir.filename_str().unwrap();
    let current_process_filename = current_dir_name.to_string() + ".rs";
    let target_path = current_dir.join("target");

    fs::create_fresh_dir(&target_path);

    println!("Compiling {} process", current_dir_name);

    let mut command = io::Command::new(fs::get_rustc_path().as_str().unwrap());

    // Link common target dirs
    for common_target_dir in fs::get_all_common_target_dirs().iter() {
        command.arg("-L");
        command.arg(common_target_dir.as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + tool_settings::get_process_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(current_process_filename);

    tool_helpers::execute_command(command);

    if !is_child_script {

        // Compile ALL the schedules. Should processes be tagged with which schedules they're in?
        for schedule_src_dir in fs::get_all_schedule_src_dirs().iter() {
            tool_helpers::run_external_application(&schedule_src_dir.join("compile"), Some(vec!["-c"]));
        }

        fs::set_is_compiling(false);
    }
}
