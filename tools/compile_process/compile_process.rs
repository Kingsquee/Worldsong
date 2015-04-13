#![feature(path_ext)]

extern crate getopts;
extern crate common;

use getopts::Options;
use std::env;
use std::fs::{File, PathExt};
use std::io::Read;
use std::process::Command;

use common::hierarchy;
use common::system;
use common::settings;

/// Compiles the process in the current directory
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
    let current_process_filename = current_dir_name.to_os_string().into_string().unwrap() + "_process.rs";
    let target_dir = current_dir.join("target");

    hierarchy::create_fresh_dir(&target_dir).unwrap();


    let mut command = Command::new(hierarchy::get_rustc_path().as_os_str().to_str().unwrap());
    command.current_dir(&current_dir);

    // Link dependencies dirs
    for path in hierarchy::get_state_dependency_dirs().iter() {
        system::link_libraries(&mut command, path)
    }

    // Link state
    system::link_libraries(&mut command, &hierarchy::get_state_target_dir());

    let config_display = system::get_compile_config(&mut command, &current_dir, &current_process_filename, &target_dir);
    command.arg("--crate-type=".to_string() + settings::get_process_lib_type());

    println!("Compiling {} process {}", current_dir_name.to_os_string().into_string().unwrap(), config_display);

    system::execute_command(&mut command);

    if !is_child_tool {
        // look for .schedule_tags
        // if doesn't exist
            // generate tags all schedules' tags
            // look for .schedule_tags again
            // if doesn't exist
                // print "Warning: This process doesn't exist in any schedules."
                // quit

        let mut schedule_tags_file = match File::open(&hierarchy::get_schedule_tags(&current_dir)) {
            Err(_) => {
                println!("No tag file found, generating tags...");
                system::run(&hierarchy::get_generate_schedule_tags_binary(), None);
                match File::open(&hierarchy::get_schedule_tags(&current_dir)) {
                    Err(e) => {
                        hierarchy::set_is_compiling(false).unwrap();
                        panic!("{}", e);
                    }
                    Ok(file) => file,
                }
            }
            Ok(file) => file,
        };

        // at this point, .schedule_tags should exist

        // parse schedule names into schedule_paths
        let mut contents = String::new();
        schedule_tags_file.read_to_string(&mut contents).unwrap();

        if contents.len() == 0 {
            println!("Warning: Process is not added to any schedules");
            return
        }

        // for each src_dir in schedule_paths
            // compile it

        // TODO: Run these commands in parallel, not in sequence? Would need to explicitly compile the scheduler from here, then.
        for line in contents.split('\n') {
            if line == "" {
                continue
            }
            let schedule_src_dir = hierarchy::get_schedules_dir().join(line);
            if schedule_src_dir.exists() {
                system::run(&schedule_src_dir.join("compile"), Some(vec!["-c"]));
            }
        }
        hierarchy::set_is_compiling(false).unwrap();
    }
}
