extern crate getopts;
extern crate environment;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;
use std::io::fs::PathExtensions;

use environment::hierarchy;
use environment::system;
use environment::settings;

/// Compiles the schedule by auto-linking all processes
fn main() {

    // Program args
    let mut is_child_tool: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("c", "child", "Run as a child compilation tool: i.e. Don't recompile dependent modules and don't modify the .is_compiling file.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("c") {
        is_child_tool = true
    };

    // Lets compile!
    if !is_child_tool {
        hierarchy::set_is_compiling(true);
    }

    let current_dir = os::self_exe_path().unwrap();

    let schedule_filename = current_dir.filename_str().unwrap().to_string() + ".rs";
    let target_path = current_dir.join("target");

    hierarchy::create_fresh_dir(&target_path);

    println!("Compiling schedule");

    let mut command = io::Command::new(hierarchy::get_rustc_path().as_str().unwrap());
    
    // Link macro dirs
    for path in hierarchy::get_all_macro_target_dirs().iter() {
        command.arg("-L").arg(path.as_str().unwrap());
    }

    // Link dependencies dirs
    for path in hierarchy::get_state_dependency_dirs().iter() {
        command.arg("-L").arg(path.as_str().unwrap());
    }
    
    // Link data structs
    for path in hierarchy::get_all_struct_target_dirs().iter() {
        command.arg("-L").arg(path.as_str().unwrap());
    }
    
    // Link state
    command.arg("-L").arg(&hierarchy::get_state_target_dir());

    // Link process target dirs
    for process_target_dir in hierarchy::get_all_process_target_dirs().iter() {
        command.arg("-L");
        command.arg(process_target_dir.as_str().unwrap());
    }

    command.arg("--out-dir").arg("./target");
    command.arg("--crate-type=".to_string() + settings::get_schedules_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(schedule_filename);

    system::execute_command(&mut command);

    if !is_child_tool {
        // Compile the scheduler
        system::run(&hierarchy::get_scheduler_src_dir().join(Path::new("compile")), Some(vec!["-c"]));
        hierarchy::set_is_compiling(false);
    }
}
