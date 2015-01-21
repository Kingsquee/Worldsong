use std::os;
use std::io;
use std::io::{IoResult, IoErrorKind};
use std::io::fs;
use std::io::fs::PathExtensions;
use std::path::Path;

pub fn create_fresh_dir(path: &Path) -> IoResult<()> {
    match fs::rmdir_recursive(path) {
        Ok(_) => (),
        Err(e) => match e.kind {
            IoErrorKind::FileNotFound => (),
            _ => { 
                return Err(e)
            }
        }
    };

    match fs::mkdir(path, io::USER_RWX) {
        Ok(_) => (),
        Err(e) => match e.kind {
            IoErrorKind::PathAlreadyExists => (),
            _ => return Err(e),
        }
    };
    
    Ok(())
}

pub fn create_fresh_file(path: &Path) -> IoResult<io::File> {
    match fs::unlink(path) {
        Ok(_) => /*println!("Removed file at {}", path.display())*/(),
        Err(e) => match e.kind {
            IoErrorKind::FileNotFound => (),
            _ => { 
                return Err(e)
            }
        }
    };

    io::File::create(path)
}

pub fn set_is_compiling(value: bool) -> IoResult<()> {
    match value {
        true => { 
            io::File::create(&get_is_compiling_tag()).unwrap();
            Ok(())
        }
        false => { 
            match fs::unlink(&get_is_compiling_tag()) {
                Ok(o) => Ok(o),
                Err(e) => match e.kind {
                    IoErrorKind::FileNotFound => Ok(()),
                    _ => return Err(e),
                }
            }
        }
    }
}

// Worldsong Modules
lazy_static!{
    static ref WORLDSONG_ROOT_DIR: Path = {
    
        let mut current_dir = os::self_exe_path().unwrap();

        let mut wsroot = None;
        'l: loop {
            let contents = fs::readdir(&current_dir).unwrap();
            for entry in contents.iter() {
                if entry.is_file() && entry.filename_str().unwrap() == ".wsroot" {
                    wsroot = Some(current_dir);
                    break 'l
                }
            }
            if !current_dir.pop() {
                break 'l
            }
        }
        
        match wsroot {
            Some(wsroot) => wsroot,
            None => panic!("ERROR: Could not find worldsong root. Was the .wsroot file removed?"),
        }
    };
}

pub fn get_worldsong_root_dir() -> Path {
    WORLDSONG_ROOT_DIR.clone()
}

// common
define_str!(COMMON_SRC_DIR, "common");
pub fn get_common_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMMON_SRC_DIR)
}

define_str!(COMMON_TARGET_DIR, COMMON_SRC_DIR!(), "/target");
pub fn get_common_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMMON_TARGET_DIR)
}

// state
define_str!(STATE_SRC_DIR, "state");
pub fn get_state_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(STATE_SRC_DIR)
}

define_str!(STATE_TARGET_DIR, STATE_SRC_DIR!(), "/target");
pub fn get_state_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(STATE_TARGET_DIR)
}

pub fn get_state_dependency_dirs() -> Vec<Path> {
    let mut vec = Vec::new();
    vec.push(get_state_target_dir().join("deps"));
    vec.push(get_state_target_dir().join("native"));
    vec
}


// structs
define_str!(STRUCTS_DIR, "structs");
pub fn get_structs_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(STRUCTS_DIR)
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


// kernel
define_str!(KERNEL_SRC_DIR, "kernel");
pub fn get_kernel_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(KERNEL_SRC_DIR)
}

define_str!(KERNEL_TARGET_DIR, KERNEL_SRC_DIR!(), "/target");
pub fn get_kernel_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(KERNEL_TARGET_DIR)
}

// scheduler
define_str!(SCHEDULER_SRC_DIR, "scheduler");

pub fn get_scheduler_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(SCHEDULER_SRC_DIR)
}

define_str!(SCHEDULER_TARGET_DIR, SCHEDULER_SRC_DIR!(), "/target");

pub fn get_scheduler_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(SCHEDULER_TARGET_DIR)
}

// schedules
define_str!(SCHEDULES_DIR, "schedules");
pub fn get_schedules_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(SCHEDULES_DIR)
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

// processes
define_str!(PROCESSES_DIR, "processes");
pub fn get_processes_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(PROCESSES_DIR)
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

// Worldsong Tools

define_str!(RUSTC_PATH, "rustc");
pub fn get_rustc_path() -> Path {
    Path::new(RUSTC_PATH)
}

define_str!(CARGO_PATH, "cargo");
pub fn get_cargo_path() -> Path {
    Path::new(CARGO_PATH)
}

define_str!(TOOLS_DIR, "tools");
pub fn get_tools_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(TOOLS_DIR)
}

define_str!(RUN_KERNEL_TOOL_SRC_DIR, TOOLS_DIR!(), "/run_kernel");
define_str!(RUN_KERNEL_TOOL_TARGET_DIR, RUN_KERNEL_TOOL_SRC_DIR!(), "/target");

pub fn get_run_kernel_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(RUN_KERNEL_TOOL_SRC_DIR)
}

pub fn get_run_kernel_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(RUN_KERNEL_TOOL_TARGET_DIR)
}

define_str!(NEW_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/new_state_struct");
pub fn get_new_state_struct_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(NEW_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(NEW_STATE_STRUCT_TOOL_TARGET_DIR, NEW_STATE_STRUCT_TOOL_SRC_DIR!(), "/target");
pub fn get_new_state_struct_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(NEW_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(COMPILE_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_state_struct");
pub fn get_compile_state_struct_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(COMPILE_STATE_STRUCT_TOOL_TARGET_DIR, COMPILE_STATE_STRUCT_TOOL_SRC_DIR!(), "/target");
pub fn get_compile_state_struct_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(COMPILE_SCHEDULER_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_scheduler");
pub fn get_compile_scheduler_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULER_TOOL_SRC_DIR)
}

define_str!(COMPILE_SCHEDULER_TOOL_TARGET_DIR, COMPILE_SCHEDULER_TOOL_SRC_DIR!(), "/target");
pub fn get_compile_scheduler_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULER_TOOL_TARGET_DIR)
}

define_str!(COMPILE_SCHEDULE_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_schedule");
pub fn get_compile_schedule_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULE_TOOL_SRC_DIR)
}

define_str!(COMPILE_SCHEDULE_TOOL_TARGET_DIR, COMPILE_SCHEDULE_TOOL_SRC_DIR!(), "/target");
pub fn get_compile_schedule_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_SCHEDULE_TOOL_TARGET_DIR)
}

define_str!(COMPILE_PROCESS_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_process");
pub fn get_compile_process_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_PROCESS_TOOL_SRC_DIR)
}

define_str!(COMPILE_PROCESS_TOOL_TARGET_DIR, COMPILE_PROCESS_TOOL_SRC_DIR!(), "/target");
pub fn get_compile_process_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_PROCESS_TOOL_TARGET_DIR)
}

define_str!(COMPILE_KERNEL_TOOL_SRC_DIR, TOOLS_DIR!(), "/compile_kernel");
pub fn get_compile_kernel_tool_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_KERNEL_TOOL_SRC_DIR)
}

define_str!(COMPILE_KERNEL_TOOL_TARGET_DIR, COMPILE_KERNEL_TOOL_SRC_DIR!(), "/target");
pub fn get_compile_kernel_tool_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(COMPILE_KERNEL_TOOL_TARGET_DIR)
}

define_str!(ADD_STATE_STRUCT_TOOL_SRC_DIR, TOOLS_DIR!(), "/add_state_struct");
pub fn get_add_state_struct_tools_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(ADD_STATE_STRUCT_TOOL_SRC_DIR)
}

define_str!(ADD_STATE_STRUCT_TOOL_TARGET_DIR, ADD_STATE_STRUCT_TOOL_SRC_DIR!(), "/target");
pub fn get_add_state_struct_tools_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(ADD_STATE_STRUCT_TOOL_TARGET_DIR)
}

define_str!(GENERATE_SCHEDULE_TAGS_SRC_DIR, TOOLS_DIR!(), "/generate_schedule_tags");
pub fn get_generate_schedule_tags_src_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(GENERATE_SCHEDULE_TAGS_SRC_DIR)
}

define_str!(GENERATE_SCHEDULE_TAGS_TARGET_DIR, GENERATE_SCHEDULE_TAGS_SRC_DIR!(), "/target");
pub fn get_generate_schedule_tags_target_dir() -> Path {
    WORLDSONG_ROOT_DIR.join(GENERATE_SCHEDULE_TAGS_TARGET_DIR)
}

// Worldsong Tags

pub fn get_schedule_tags(process_dir: &Path) -> Path {
    process_dir.join(".schedule_tags")
}

pub fn get_generate_schedule_tags_binary() -> Path {
    get_generate_schedule_tags_target_dir().join("generate_schedule_tags")
}

pub fn get_is_compiling_tag() -> Path {
    WORLDSONG_ROOT_DIR.join(".is_compiling")
}
