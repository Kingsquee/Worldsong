#[macro_use]
extern crate lazy_static;

use std::env;
use std::io;
use std::fs;
use std::fs::{File/*, PathExt*/};
use std::path::{PathBuf, Path};



///////////////////////////////
// Worldsong Directory Names //
///////////////////////////////

// Macro for defining constant strings
macro_rules! const_str (
    ($name: ident, $($arg: expr), +)
    =>
    (
        macro_rules! $name (
            () => (concat!($($arg), +))
        );
        #[allow(dead_code)]
        const $name: &'static str = $name!();
    );
);

macro_rules! concat_path (
    ($($arg: expr), +)
    =>
    (
        (concat!($(concat!($arg, "/")), +)
        );
    )
);

macro_rules! binary_name (
    ($name: expr) => (
        format!("{}{}{}", $name, env::consts::EXE_SUFFIX, env::consts::EXE_EXTENSION)
    )
);

macro_rules! dylib_name (
    ($name: expr) => (
        format!("{}{}{}{}", env::consts::DLL_PREFIX, $name, env::consts::DLL_SUFFIX, env::consts::DLL_EXTENSION)
    )
);

// GENERIC DIR NAMES
const_str!(SRC,                 "src");
const_str!(TARGET,              "target");
const_str!(PROJECTS,            "projects");
const_str!(TAGS,                ".tags");
const_str!(DEPENDENCIES,        "dependencies");
const_str!(LIBRARIES,           "libraries");

// CARGO DIR NAMES
#[cfg(debug_assertions)]
const_str!(CARGO_TARGET_DIR,    "target/debug");
#[cfg(not(debug_assertions))]
const_str!(CARGO_TARGET_DIR,    "target/release");

const_str!(DEPS,                "deps");
const_str!(NATIVE,              "native");

// RUST TOOLING NAMES
const_str!(RUSTC,               "rustc");
const_str!(CARGO,               "cargo");




///////////////////////////////////
// Worldsong Directory Structure //
///////////////////////////////////

//TODO: Cannot display error messages since this does some thread shenanigans.
lazy_static!{
    static ref WORLDSONG_ROOT_DIR: PathBuf = {

        let mut current_dir = env::current_exe().unwrap();
        current_dir.pop();

        let mut wsroot;
        'l: loop {
            for entry in fs::read_dir(&current_dir).unwrap() {
                let entry = entry.unwrap().path();
                if entry.file_name().unwrap() == ".wsroot" {
                    wsroot = Some(current_dir);
                    break 'l
                }
            }
            if !current_dir.pop() {
                wsroot = None;
                break 'l
            }
        }
        match wsroot {
            Some(wsroot) => wsroot,
            None => panic!("ERROR: Could not find worldsong root directory. Was the .wsroot file removed?"),
        }


    };
}

pub fn get_worldsong_root_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.clone()
}

pub fn get_all_project_dirs() -> Vec<PathBuf> {
    let projects_dir = WORLDSONG_ROOT_DIR.join(PROJECTS!());
    let mut dirs = Vec::new();

    for entry in fs::read_dir(&projects_dir).unwrap() {
        let entry = entry.unwrap().path();
        dirs.push(entry)
    }
    dirs
}

// App dirs are always one directory below the WSROOT dir
// WSROOT DIR   = /a/b/c
// CURRENT DIR  = /a/b/c/d/e
// ergo:
// APP DIR      = /a/b/c/d
// So, we'll just bubble the current dir up the directory path until we find the worldsong root, and return the previous current directory.

pub fn get_current_project_dir() -> PathBuf {
    let mut current_dir = env::current_dir().unwrap();
    let mut previous_dir = current_dir.clone();
    let projects_dir_path = WORLDSONG_ROOT_DIR.join(PROJECTS!());

    'l: loop {
        if &current_dir == &projects_dir_path {
            return previous_dir
        }
        previous_dir = current_dir.clone();
        current_dir.pop();
    }
}

pub fn get_project_dir(project_name: &str) -> PathBuf {
    WORLDSONG_ROOT_DIR.join(project_name)
}

pub fn get_module_src_dir(project_dir: &Path, module_name: &str) -> PathBuf {
    project_dir.join(SRC!()).join(module_name)
}

pub fn get_module_all_src_paths(project_dir: &Path, module_name: &str) -> Vec<PathBuf> {
    let dir = get_module_src_dir(project_dir, module_name);
    let mut files: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap().path();
        if let Some(extension) = entry.extension() {
            if extension != "rs" { continue }
            files.push(entry.clone());
        }
    }
    files
}


pub fn get_module_target_dir(project_dir: &Path, module_name: &str) -> PathBuf {
    project_dir.join(concat_path!(TARGET!())).join(module_name)
}

pub fn get_module_target_bin(project_dir: &Path, module_name: &str) -> PathBuf {
    project_dir.join(concat_path!(TARGET!())).join(module_name).join(binary_name!(module_name))
}

// Project Dependencies Target

pub fn get_dependencies_all_library_dirs(project_dir: &Path) -> Vec<PathBuf> {
    let dir = project_dir.join(concat_path!(DEPENDENCIES!(), LIBRARIES!()));
    let mut dirs: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap().path();
        //if entry.is_dir() {
            dirs.push(entry.clone());
        //}
    }
    dirs
}

pub fn get_dependencies_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(concat_path!(DEPENDENCIES!()))
}

pub fn get_dependencies_all_target_dirs(project_dir: &Path) -> Vec<PathBuf> {
    vec![
        get_dependencies_deps_target_dir(project_dir),
        get_dependencies_native_target_dir(project_dir)
    ]
}

pub fn get_dependencies_deps_target_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(concat_path!(DEPENDENCIES!(), CARGO_TARGET_DIR!(), DEPS!()))
}

pub fn get_dependencies_native_target_dir(project_dir: &Path) -> PathBuf {
    project_dir.join(concat_path!(DEPENDENCIES!(), CARGO_TARGET_DIR!(), NATIVE!()))
}

// Global Dependencies Target

pub fn get_global_dependencies_all_library_dirs() -> Vec<PathBuf> {
    let dir = WORLDSONG_ROOT_DIR.join(concat_path!(DEPENDENCIES!(), LIBRARIES!()));
    let mut dirs: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(&dir).unwrap() {
        let entry = entry.unwrap().path();
        //if entry.is_dir() {
            dirs.push(entry.clone());
        //}
    }
    dirs
}

pub fn get_global_dependencies_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(concat_path!(DEPENDENCIES!()))
}

pub fn get_global_dependencies_all_target_dirs() -> Vec<PathBuf> {
    vec![
        get_global_dependencies_deps_target_dir(),
        get_global_dependencies_native_target_dir()
    ]
}

pub fn get_global_dependencies_deps_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(concat_path!(DEPENDENCIES!(), CARGO_TARGET_DIR!(), DEPS!()))
}

pub fn get_global_dependencies_native_target_dir() -> PathBuf {
    WORLDSONG_ROOT_DIR.join(concat_path!(DEPENDENCIES!(), CARGO_TARGET_DIR!(), NATIVE!()))
}

// Specific base software
pub fn get_rustc_binary_path() -> PathBuf {
    PathBuf::from(RUSTC!())
}

pub fn get_cargo_binary_path() -> PathBuf {
    PathBuf::from(CARGO!())
}

pub fn get_temp_dir(project_dir: &Path, module_name: &str) -> PathBuf {
    env::temp_dir().join(project_dir).join(".tmp").join(module_name)
}

////////////////////
// Worldsong Tags //
////////////////////

pub fn get_global_tag_path(project_dir: &Path, tag_name: &str) -> PathBuf {
    project_dir.join(TAGS!()).join(tag_name)
}

pub fn get_file_tag_path(project_dir: &Path, file_name: &str, tag_name: &str) -> PathBuf {
    project_dir.join(TAGS!()).join(tag_name).join(file_name)
}

pub fn set_boolean_tag(tag_path: &Path, value: bool) -> Result<(), io::Error> {
    match value {
        true => {
            File::create(&tag_path).unwrap();
            Ok(())
        }
        false => {
            match fs::remove_file(&tag_path) {
                Ok(o) => Ok(o),
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound => Ok(()),
                    _ => return Err(e),
                }
            }
        }
    }
}

pub fn get_boolean_tag(tag_path: &Path) -> Result<bool, io::Error> {
    match fs::File::open(tag_path) {
        Ok(_) => Ok(true),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => Ok(false),
            _ => Err(e)
        }
    }
}

////////////////////////////
// Compiler Configuration //
////////////////////////////

const_str!(CONFIG_EXT, ".config");

pub fn get_module_compile_config_path(module_dir: &Path) -> PathBuf {
    let config_name = format!("{}{}", module_dir.file_stem().unwrap().to_str().unwrap(), CONFIG_EXT);
    module_dir.join(config_name)
}

pub fn get_file_compile_config_path(src_file_path: &Path) -> PathBuf {
    let module_dir = src_file_path.parent().unwrap();
    let config_name = format!("{}{}", src_file_path.file_stem().unwrap().to_str().unwrap(), CONFIG_EXT);
    module_dir.join(config_name)
}


//////////
// Misc //
//////////

pub fn create_fresh_dir(path: &Path) -> io::Result<()> {
    match fs::remove_dir_all(path) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => (),
            _ => {
                return Err(e)
            }
        }
    };

    match fs::create_dir_all(path) {
        Ok(_) => (),
        Err(e) => match e.kind() {
            io::ErrorKind::AlreadyExists => (),
            _ => return Err(e),
        }
    };

    Ok(())
}

pub fn create_file_all(path: &Path) -> io::Result<File> {
    match fs::File::create(path) {
        Ok(f) => Ok(f),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                let mut parent_dir = PathBuf::from(path);
                parent_dir.pop();

                match fs::create_dir_all(parent_dir) {
                    Ok(_) => {
                        fs::File::create(path)
                    }
                    Err(e) => Err(e)
                }
            }
            _ => Err(e)
        }
    }
}