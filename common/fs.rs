use std::os;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::path::Path;

// TODO: return IoResult<()>
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

// TODO: return IoResult<()>
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

pub fn get_state_dir() -> Path {
    get_worldsong_root_dir().join("state")
}

pub fn get_all_common_target_dirs() -> Vec<Path> {
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

pub fn get_run_kernel_script_src_dir() -> Path {
    get_compile_scripts_dir().join("run_kernel")
}

pub fn get_run_kernel_script_target_dir() -> Path {
    get_run_kernel_script_src_dir().join("run_kernel")
}

pub fn get_new_state_struct_script_src_dir() -> Path {
    get_compile_scripts_dir().join("new_state_struct")
}

pub fn get_new_state_struct_script_target_dir() -> Path {
    get_new_state_struct_script_src_dir().join("target")
}

pub fn get_compile_state_struct_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_state_struct")
}

pub fn get_compile_state_struct_script_target_dir() -> Path {
    get_compile_state_struct_script_src_dir().join("target")
}

pub fn get_compile_scheduler_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_scheduler")
}

pub fn get_compile_scheduler_script_target_dir() -> Path {
    get_compile_scheduler_script_src_dir().join("target")
}

pub fn get_compile_schedule_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_schedule")
}

pub fn get_compile_schedule_script_target_dir() -> Path {
    get_compile_schedule_script_src_dir().join("target")
}

pub fn get_compile_process_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_process")
}

pub fn get_compile_process_script_target_dir() -> Path {
    get_compile_process_script_src_dir().join("target")
}

pub fn get_compile_kernel_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_kernel")
}

pub fn get_compile_kernel_script_target_dir() -> Path {
    get_compile_kernel_script_src_dir().join("target")
}

pub fn get_compile_common_script_src_dir() -> Path {
    get_compile_scripts_dir().join("compile_common")
}

pub fn get_compile_common_script_target_dir() -> Path {
    get_compile_common_script_src_dir().join("target")
}


pub fn get_is_compiling_file() -> Path {
    get_worldsong_root_dir().join(".iscompiling")
}
