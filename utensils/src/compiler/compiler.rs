extern crate worldsong_hierarchy;
extern crate worldsong_config;
extern crate system;
extern crate getopts;
extern crate regex;
extern crate toml;

mod compile_dependencies;
mod compile_state;
mod compile_kernel;
mod compile_process;
mod compile_schedule;
mod compile_scheduler;
mod generate_schedules_tags;


use std::env;
use std::path::{Path, PathBuf};
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();


    let opts = Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let app_dir = worldsong_hierarchy::get_current_project_dir();

    let is_compiling = worldsong_hierarchy::get_global_tag_path(&app_dir, "is_compiling");

    // If we didn't pass any arguments, we're either being run from the project root directory or a module.
    if matches.free.is_empty() {
        let app_dir_name = app_dir.file_name().unwrap().to_str().unwrap();

        match env::current_dir().unwrap().file_name().unwrap().to_str().unwrap() {
            "dependencies" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_dependencies::exec(&app_dir);
                compile_state::exec(&app_dir);
                compile_all_processes(&app_dir);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "state" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_state::exec(&app_dir);
                compile_all_processes(&app_dir);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "kernel" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "processes" => {
                println!("NOTE: You're compiling all the processes. If you just want to compile a single process, pass its file path as an argument.");
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_all_processes(&app_dir);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "schedules" => {
                println!("NOTE: You're compiling all the schedules. If you just want to compile a single schedule, pass its file path as an argument.");
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_all_schedules(&app_dir);
                generate_schedules_tags::exec(&app_dir);
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "scheduler" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            x => {
                if x == app_dir_name {
                    println!("Compiling the entire {} project.", app_dir_name);
                    worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                    compile_dependencies::exec(&app_dir);
                    compile_state::exec(&app_dir);
                    compile_all_processes(&app_dir);
                    compile_all_schedules(&app_dir);
                    compile_scheduler::exec(&app_dir);
                    compile_kernel::exec(&app_dir);
                    worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
                } else {
                    println!("ERROR: Could not determine what worldsong module you want compiled, found {}", x);
                }
            }
        }
    } else {
        let src_path = PathBuf::from(&matches.free[0]);
        let parent_dir_name = src_path.parent().unwrap().file_name().unwrap().to_str().unwrap();

        match parent_dir_name {
            "dependencies" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_dependencies::exec(&app_dir);
                compile_state::exec(&app_dir);
                compile_all_processes(&app_dir);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "state" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_state::exec(&app_dir);
                compile_all_processes(&app_dir);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "kernel" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_kernel::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "processes" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_process::exec(&app_dir, &src_path);
                compile_all_schedules(&app_dir);
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "schedules" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_schedule::exec(&app_dir, &src_path);
                generate_schedules_tags::exec(&app_dir);
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            "scheduler" => {
                worldsong_hierarchy::set_boolean_tag(&is_compiling, true).unwrap();
                compile_scheduler::exec(&app_dir);
                worldsong_hierarchy::set_boolean_tag(&is_compiling, false).unwrap();
            }
            x => {
                println!("ERROR: Could not determine what worldsong module you want compiled, found {}", x);
            }
        }
    }
}

fn compile_all_processes(app_dir: &Path) {
    for src_path in worldsong_hierarchy::get_module_all_src_files(&app_dir, "processes").iter() {
        compile_process::exec(&app_dir, src_path);
    }
}

fn compile_all_schedules(app_dir: &Path) {
    for src_path in worldsong_hierarchy::get_module_all_src_files(&app_dir, "schedules").iter() {
        compile_schedule::exec(&app_dir, src_path);
    }
}