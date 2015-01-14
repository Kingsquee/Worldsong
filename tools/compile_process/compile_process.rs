extern crate getopts;
extern crate environment;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;

use environment::hierarchy;
use environment::system;
use environment::settings;

/// Compiles the process in the current directory
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
    let current_dir_name = current_dir.filename_str().unwrap();
    let current_process_filename = current_dir_name.to_string() + "_process.rs";
    let target_path = current_dir.join("target");

    match hierarchy::create_fresh_dir(&target_path) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    };

    println!("Compiling {} process", current_dir_name);

    let mut command = io::Command::new(hierarchy::get_rustc_path().as_str().unwrap());
    command.cwd(&current_dir);

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
    
    command.arg("--out-dir").arg(target_path.as_str().unwrap());
    command.arg("--crate-type=".to_string() + settings::get_process_lib_type());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(current_process_filename);

    system::execute_command(&mut command);

    if !is_child_tool {
        // Compile ALL the schedules. 
        
        // TODO: Run these commands in parallel, not in sequence.
        
        // TODO: Processes should be optionally tagged with which schedules they're in.
        //  If it's not run with --tag, compile all the schedules.
        
        for schedule_src_dir in hierarchy::get_all_schedule_src_dirs().iter() {
            system::run(
                &schedule_src_dir.join("compile"), 
                Some(vec!["-c"])
            );
        }
        hierarchy::set_is_compiling(false);
    }
}
