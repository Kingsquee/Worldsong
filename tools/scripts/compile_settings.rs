#![allow(dead_code)]

use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::path::Path;

/*
[Sunday, November 30, 2014] [12:28:23 ▾] <Kingsqueee>   Is there a way I can tell rustup.sh to install to a local directory?
[Sunday, November 30, 2014] [12:28:47 ▾] <Kingsqueee>   I'd like a 'portable' compiler
[Sunday, November 30, 2014] [12:29:08 ▾] <geofft>   Kingsqueee: yeah, it takes --prefix
[Sunday, November 30, 2014] [12:29:36 ▾] <geofft>   I run rustup with --prefix=/home/geofft/b because I don't like installing stuff globally
[Sunday, November 30, 2014] [12:29:50 ▾] <geofft>   so I have to export PATH=$PATH:/home/geofft/b/bin and export LD_LIBRARY_PATH=/home/geofft/b/lib
[Sunday, November 30, 2014] [12:29:53 ▾] <geofft>   and it works
[Sunday, November 30, 2014] [12:31:00 ▾] <Kingsqueee>   geofft: awesome!
*/

pub fn run_external_application(app: &Path, args: Option<Vec<&str>>) {
    println!("Running {}", app.display());
    let mut command = io::Command::new(app.clone());
    if args.is_some() {
        for arg in args.unwrap().iter() {
            command.arg(arg);
        }
    }
    command.cwd(&app.dir_path());
    execute_command(command);
}

pub fn execute_command(command: io::Command) {
    // Try to run this thing
    let result = match command.output() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    // If it ran, how'd it do?
    match result.status.success() {
        true => {
            println!("{}", String::from_utf8(result.output).unwrap());
        }
        false => {
            panic!("{}", String::from_utf8(result.error).unwrap());
        }
    };
}

pub fn create_fresh_dir(path: &Path) {

    match fs::rmdir_recursive(path) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }

    match fs::mkdir(path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}

pub fn set_is_compiling(value: bool) {
    if value == true {
        match io::File::create(&get_is_compiling_file()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    } else {
        match fs::unlink(&get_is_compiling_file()) {
            Ok(_) => (),
            Err(e) => panic!("{}", e)
        }
    }
}


// What directories store the different Worldsong crates?

pub fn get_thirdparty_tools_dir() -> Path {
    get_tools_dir().join("thirdparty")
}

pub fn get_rustc_target_dir() -> Path {
    get_thirdparty_tools_dir().join("rustc").join("bin")
}

pub fn get_rustc_lib_dir() -> Path {
    get_thirdparty_tools_dir().join("rustc").join("lib")
}

pub fn get_rustc_path() -> Path {
    Path::new("rustc")
    //get_rustc_target_dir().join("rustc")
}

pub fn get_cargo_path() -> Path {
    //Path::new("cargo")
    get_thirdparty_tools_dir().join("cargo").join("bin").join("cargo")
}

// TODO: Only do this Once
// http://doc.rust-lang.org/std/sync/struct.Once.html
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

pub fn get_common_src_dir() -> Path {
    get_worldsong_root_dir().join("common")
}

pub fn get_common_target_dirs() -> Vec<Path> {
    let mut dirs: Vec<Path> = Vec::new();
    dirs.push(get_common_src_dir().join("target"));
    dirs.push(get_common_src_dir().join("target/deps"));
    dirs.push(get_common_src_dir().join("target/native"));
    dirs
}

pub fn get_kernel_src_dir() -> Path {
    get_worldsong_root_dir().join("kernel")
}

pub fn get_kernel_target_dir() -> Path {
    get_kernel_src_dir().join("target")
}

pub fn get_scheduler_src_dir() -> Path {
    get_worldsong_root_dir().join("scheduler")
}

pub fn get_scheduler_target_dir() -> Path {
    get_scheduler_src_dir().join("target")
}

pub fn get_schedules_dir() -> Path {
    get_worldsong_root_dir().join("schedules")
}

pub fn get_all_schedule_src_dirs() -> Vec<Path> {
    let mut dirs: Vec<Path> = Vec::new();
    let contents = fs::readdir(&get_schedules_dir()).unwrap();
    for entry in contents.iter() {
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

pub fn get_all_schedule_target_dirs() -> Vec<Path> {
    let mut dirs = get_all_schedule_src_dirs();
    for schedule_path in dirs.iter_mut() {
        schedule_path.push("target")
    }
    dirs
}

pub fn get_processes_dir() -> Path {
    get_worldsong_root_dir().join("processes")
}

pub fn get_all_process_src_dirs() -> Vec<Path> {
    let processes_dir = get_processes_dir();
    let mut dirs: Vec<Path> = Vec::new();

    for entry in fs::readdir(&processes_dir).unwrap().iter() {
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

pub fn get_all_process_target_dirs() -> Vec<Path> {
    let mut dirs = get_all_process_src_dirs();
    for entry in dirs.iter_mut() {
        entry.push("target")
    }
    dirs
}

pub fn get_tools_dir() -> Path {
    get_worldsong_root_dir().join("tools")
}

pub fn get_compile_scripts_dir() -> Path {
    get_tools_dir().join("scripts")
}

pub fn get_is_compiling_file() -> Path {
    get_worldsong_root_dir().join(".iscompiling")
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
