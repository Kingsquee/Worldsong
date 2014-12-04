#![allow(dead_code)]

use std::os;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::path::Path;


// What directories store the different Worldsong crates?

pub fn get_thirdparty_tools_dir() -> Path {
    get_tools_dir().join("thirdparty")
}

pub fn get_rustc_path() -> Path {
    get_thirdparty_tools_dir().join("latest_rust").join("bin").join("rustc")
}

pub fn get_cargo_path() -> Path {
    //Path::new("cargo")
    get_thirdparty_tools_dir().join("cargo").join("bin").join("cargo")
}

pub fn get_worldsong_root_dir() -> Path {
    let mut current_dir = os::self_exe_path().unwrap();

    loop {
        let contents = fs::readdir(&current_dir).unwrap();
        for entry in contents.iter() {
            if entry.is_file() && entry.filename_str().unwrap() == ".wsroot" {
                //println!("Worldsong root is {}", current_dir.display());
                return current_dir
            }
        }
        if !current_dir.pop() {
            panic!("ERROR: Could not find worldsong root. Was the .wsroot file removed?");
        }
    }
}

pub fn get_common_dir() -> Path {
    get_worldsong_root_dir().join("common")
}

pub fn get_kernel_dir() -> Path {
    get_worldsong_root_dir().join("kernel")
}

pub fn get_scheduler_dir() -> Path {
    get_worldsong_root_dir().join("scheduler")
}

pub fn get_schedules_dir() -> Path {
    get_worldsong_root_dir().join("schedules")
}

pub fn get_tools_dir() -> Path {
    get_worldsong_root_dir().join("tools")
}

pub fn get_compile_scripts_dir() -> Path {
    get_tools_dir().join("scripts")
}

pub fn get_all_schedule_root_dirs() -> Vec<Path> {
    let mut schedules_dirs: Vec<Path> = Vec::new();
    let contents = fs::readdir(&get_schedules_dir()).unwrap();
    for entry in contents.iter() {
        if entry.is_dir() {
            schedules_dirs.push(entry.clone());
        }
    }

    schedules_dirs
}

pub fn get_all_schedule_src_dirs() -> Vec<Path> {
    let mut schedules_src_dirs = get_all_schedule_root_dirs();
    for schedule_path in schedules_src_dirs.iter_mut() {
        schedule_path.push("schedule")
    }
    schedules_src_dirs
}

pub fn get_schedule_root_dir(schedule_name: &str) -> Path {
    get_schedules_dir().join(schedule_name)
}

pub fn get_schedule_src_dir(schedule_name: &str) -> Path {
    get_schedule_root_dir(schedule_name).join("schedule")
}

pub fn get_processes_dir(schedule_name: &str) -> Path {
    get_schedule_root_dir(schedule_name).join("processes")
}

pub fn get_all_process_dirs(schedule_name: &str) -> Vec<Path> {
    let mut processes_paths: Vec<Path> = Vec::new();
    let contents = fs::readdir(&get_processes_dir(schedule_name)).unwrap();
    for entry in contents.iter() {
        if entry.is_dir() {
            processes_paths.push(entry.clone());
        }
    }

    processes_paths
}


// How should each library crate be compiled? Statically or dynamically?

pub fn get_process_lib_type() -> &'static str {
    "dylib"
}

pub fn get_schedules_lib_type() -> &'static str {
    "dylib"
}

pub fn get_scheduler_lib_type() -> &'static str {
    "dylib"
}

pub fn get_common_lib_type() -> &'static str {
    "dylib"
}
