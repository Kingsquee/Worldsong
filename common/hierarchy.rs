use std::os;
use std::io;
use std::io::{IoResult, IoErrorKind};
use std::io::fs;
use std::io::fs::PathExtensions;
use std::path::Path;

pub fn create_fresh_dir(path: &Path) -> IoResult<()> {
    let mut result: IoResult<()> = Ok(());
    
    result = fs::rmdir_recursive(path);
    if result.is_err() { return result }

    result = fs::mkdir(path, io::USER_RWX);
    if result.is_err() { return result }
    
    result
}

//TODO: Make this safe
pub fn set_is_compiling(value: bool) {
    match value {
        true => { 
            io::File::create(&get_is_compiling_tag()).unwrap();
        }
        false => { 
            match fs::unlink(&get_is_compiling_tag()) {
                Ok(_) => (),
                Err(e) => match e.kind {
                    IoErrorKind::FileNotFound => (),
                    _ => panic!("{}", e),
                }
            }
        }
    }
}

//TODO: Make this safe
pub fn set_state_cargo_toml_needs_regen(value: bool) {
    match value { 
        true => { io::File::create(&get_state_cargo_toml_needs_regen_tag()).unwrap(); }
        false => { 
            match fs::unlink(&get_state_cargo_toml_needs_regen_tag()) {
                Ok(_) => (),
                Err(e) => match e.kind {
                    IoErrorKind::FileNotFound => (),
                    _ => panic!("{}", e),
                }
            }
        }
    }
}

//TODO: Make this safe
pub fn set_state_src_needs_regen(value: bool) {
    match value { 
        true => { io::File::create(&get_state_src_needs_regen_tag()).unwrap(); }
        false => { 
            match fs::unlink(&get_state_src_needs_regen_tag()) {
                Ok(_) => (),
                Err(e) => match e.kind {
                    IoErrorKind::FileNotFound => (),
                    _ => panic!("{}", e),
                }
            }
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
    Path::new("cargo")
    //get_thirdparty_tools_dir().join("cargo").join("bin").join("cargo")
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

pub fn get_common_target_dir() -> Path {
    get_common_src_dir().join("target")
}

pub fn get_state_src_dir() -> Path {
    get_worldsong_root_dir().join("state")
}

pub fn get_state_target_dir() -> Path {
    get_state_src_dir().join("target")
}

pub fn get_dependencies_dirs() -> Vec<Path> {
    let mut vec = Vec::new();
    vec.push(get_state_target_dir().join("deps"));
    vec.push(get_state_target_dir().join("native"));
    vec
}

pub fn get_structs_dir() -> Path {
    get_worldsong_root_dir().join("structs")
}

pub fn get_all_struct_src_dirs() -> Vec<Path> {
    let structs_dir = get_structs_dir();
    let mut dirs: Vec<Path> = Vec::new();

    for entry in fs::readdir(&structs_dir).unwrap().iter() {
        if entry.is_dir() {
            dirs.push(entry.clone());
        }
    }
    dirs
}

pub fn get_all_struct_target_dirs() -> Vec<Path> {
    let mut dirs = get_all_struct_src_dirs();
    for entry in dirs.iter_mut() {
        entry.push("target")
    }
    dirs
}

pub fn get_all_struct_dep_dirs() -> Vec<Path> {
    let mut target_dirs = get_all_struct_target_dirs();
    let mut dirs = Vec::new();
    for entry in target_dirs.iter() {
        dirs.push(entry.clone().join("deps"));
        dirs.push(entry.clone().join("native"));
    }
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

pub fn get_compile_builders_dir() -> Path {
    get_tools_dir().join("builders")
}

pub fn get_run_kernel_builder_src_dir() -> Path {
    get_compile_builders_dir().join("run_kernel")
}

pub fn get_run_kernel_builder_target_dir() -> Path {
    get_run_kernel_builder_src_dir().join("run_kernel")
}

pub fn get_new_state_struct_builder_src_dir() -> Path {
    get_compile_builders_dir().join("new_state_struct")
}

pub fn get_new_state_struct_builder_target_dir() -> Path {
    get_new_state_struct_builder_src_dir().join("target")
}

pub fn get_compile_state_struct_builder_src_dir() -> Path {
    get_compile_builders_dir().join("compile_state_struct")
}

pub fn get_compile_state_struct_builder_target_dir() -> Path {
    get_compile_state_struct_builder_src_dir().join("target")
}

pub fn get_compile_scheduler_builder_src_dir() -> Path {
    get_compile_builders_dir().join("compile_scheduler")
}

pub fn get_compile_scheduler_builder_target_dir() -> Path {
    get_compile_scheduler_builder_src_dir().join("target")
}

pub fn get_compile_schedule_builder_src_dir() -> Path {
    get_compile_builders_dir().join("compile_schedule")
}

pub fn get_compile_schedule_builder_target_dir() -> Path {
    get_compile_schedule_builder_src_dir().join("target")
}

pub fn get_compile_process_builder_src_dir() -> Path {
    get_compile_builders_dir().join("compile_process")
}

pub fn get_compile_process_builder_target_dir() -> Path {
    get_compile_process_builder_src_dir().join("target")
}

pub fn get_compile_kernel_builder_src_dir() -> Path {
    get_compile_builders_dir().join("compile_kernel")
}

pub fn get_compile_kernel_builder_target_dir() -> Path {
    get_compile_kernel_builder_src_dir().join("target")
}

pub fn get_add_state_struct_builders_dir() -> Path {
    get_compile_builders_dir().join("add_state_struct")
}

pub fn get_add_state_struct_builders_target_dir() -> Path {
    get_add_state_struct_builders_dir().join("target")
}

//TODO: .is_compiling
pub fn get_is_compiling_tag() -> Path {
    get_worldsong_root_dir().join(".iscompiling")
}

pub fn get_state_cargo_toml_needs_regen_tag() -> Path {
    get_state_src_dir().join(".cargo_toml_needs_regen")
}

pub fn get_state_src_needs_regen_tag() -> Path {
    get_state_src_dir().join(".src_needs_regen")
}
