extern crate worldsong_hierarchy;
extern crate regex;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::env::consts;
use std::fs;
use std::fs::File;
use std::io::{Read, ErrorKind};
use regex::Regex;

/*
[Sunday, November 30, 2014] [12:28:23 ▾] <Kingsqueee>   Is there a way I can tell rustup.sh to install to a local directory?
[Sunday, November 30, 2014] [12:28:47 ▾] <Kingsqueee>   I'd like a 'portable' compiler
[Sunday, November 30, 2014] [12:29:08 ▾] <geofft>   Kingsqueee: yeah, it takes --prefix
[Sunday, November 30, 2014] [12:29:36 ▾] <geofft>   I run rustup with --prefix=/home/geofft/b because I don't like installing stuff globally
[Sunday, November 30, 2014] [12:29:50 ▾] <geofft>   so I have to export PATH=$PATH:/home/geofft/b/bin and export LD_LIBRARY_PATH=/home/geofft/b/lib
[Sunday, November 30, 2014] [12:29:53 ▾] <geofft>   and it works
[Sunday, November 30, 2014] [12:31:00 ▾] <Kingsqueee>   geofft: awesome!
*/

pub fn run(app: &Path, args: Option<Vec<&str>>) {
    println!("Running {}", app.display());
    let mut command = Command::new(app.clone());
    if args.is_some() {
        for arg in args.unwrap().iter() {
            command.arg(arg);
        }
    }
    command.current_dir(&app.parent().unwrap());
    execute_command(&mut command);
}

pub fn execute_command(command: &mut Command) {

    //println!("{:?}", command);
    // Try to run this thing
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    let mut result = match command.spawn() {
        Ok(r) => r,
        Err(e) => panic!("Failed to run: {}", e),
    };

    // If it ran, how'd it do?

    if !result.wait().unwrap().success() {
        panic!("Build failed");
    };
}

pub fn get_default_rustc_flags() -> Vec<&'static str> {
    vec!["-C", "opt-level=3", "-C", "debuginfo=2"]
}

pub fn get_compile_config(compiler_config_path: &Path, command: &mut Command) {
    match File::open(compiler_config_path) {
        Err(_) => {
            command.args(&get_default_rustc_flags());
        }
        Ok(ref mut file) => {
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Err(_) => {
                    command.args(&get_default_rustc_flags());
                }
                Ok(_) => {
                    if file_contents.is_empty() /*|| file_contents.is_whitespace()*/ {
                        command.args(&get_default_rustc_flags());
                    } else {
                        for line in file_contents.lines_any() {
                            for arg in line.split(' ') {
                                command.arg(arg);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn extract_library_name_from_file_name(lib_path: &Path) -> String {
    let lib_name = lib_path.file_stem().unwrap().to_str().unwrap();
    //println!("lib name is {}", lib_name);
    let lib_name_regex = Regex::new(&format!("{}{}", consts::DLL_PREFIX, r"(\w*)")).unwrap();
    match lib_name_regex.captures(lib_name).unwrap().at(1) {
        Some(cap) => return cap.to_string(),
        None => panic!("ERROR: invalid library: {}", lib_name),
    }
}

pub fn link_libraries(command: &mut Command, lib_dir: &Path) {
    command.arg("-L").arg(lib_dir);
    for entry in fs::read_dir(lib_dir).unwrap() {
        let lib_path = entry.unwrap().path();
        let name = extract_library_name_from_file_name(&lib_path);
        command.arg("--extern").arg(&format!("{}={}", name, lib_path.to_str().unwrap()));
    }
}

pub fn rustc_compile_bin(project_dir: &Path, dep_dirs: &Vec<PathBuf>, src_file_path: &Path, compiler_config_path: &Path) {

    let module_name = src_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    println!("Compiling {}.", src_file_path.file_stem().unwrap().to_str().unwrap());
    let target_dir = &worldsong_hierarchy::get_module_target_dir(&project_dir, &module_name);
    fs::create_dir_all(target_dir).unwrap();

    let mut command = Command::new(worldsong_hierarchy::get_rustc_binary_path());

    for dir in dep_dirs.iter() {
        link_libraries(&mut command, dir)
    }

    get_compile_config(compiler_config_path, &mut command);

    command.arg("--out-dir").arg(target_dir);
    conditional_rustc_release_flags(&mut command);
    command.arg(src_file_path);

    //println!("{:?}", command);

    execute_command(&mut command);
}

pub fn rustc_compile_lib(project_dir: &Path, dep_dirs: &Vec<PathBuf>, src_file_path: &Path, compiler_config_path: &Path) {

    let module_name = src_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    println!("Compiling {}.", src_file_path.file_stem().unwrap().to_str().unwrap());
    let target_dir = &worldsong_hierarchy::get_module_target_dir(&project_dir, &module_name);
    fs::create_dir_all(target_dir).unwrap();

    let mut command = Command::new(worldsong_hierarchy::get_rustc_binary_path());
    command.arg("--print").arg("crate-name").arg(src_file_path);

    // clean target dir of libraries with the same crate name
    let output = command.output().unwrap();
    let crate_name = String::from_utf8_lossy(&output.stdout).trim().to_string();

    for entry in fs::read_dir(target_dir).unwrap() {
        let entry = entry.unwrap().path();
        if fs::metadata(&entry).unwrap().is_file() {
            let entry_name = entry.file_stem().unwrap().to_str().unwrap().trim_left_matches(consts::DLL_PREFIX).to_string();

            //println!("{} == {}? {}", &entry_name, &crate_name, &entry_name == &crate_name);

            if &entry_name == &crate_name {
                fs::remove_file(entry).unwrap();
            }
        }
    }

    let mut command = Command::new(worldsong_hierarchy::get_rustc_binary_path());

    for dir in dep_dirs.iter() {
        link_libraries(&mut command, dir)
    }

    get_compile_config(compiler_config_path, &mut command);

    command.arg("--out-dir").arg(target_dir);
    conditional_rustc_release_flags(&mut command);
    command.arg(src_file_path);

    //println!("{:?}", command);

    execute_command(&mut command);
}

#[cfg(not(debug_assertions))]
fn conditional_rustc_release_flags(command: &mut Command) {
    command.arg("-C").arg("--opt-level=3");
}

#[cfg(debug_assertions)]
#[allow(unused_variables)]
fn conditional_rustc_release_flags(command: &mut Command) {

}

pub fn cargo_compile(cargo_project_path: &Path) {

    println!("Compiling {}", cargo_project_path.file_stem().unwrap().to_str().unwrap());

    let mut command = Command::new(worldsong_hierarchy::get_cargo_binary_path());
    command.arg("build");
    command.arg("--manifest-path");
    command.arg(cargo_project_path.join("Cargo.toml"));

    conditional_cargo_release_flags(&mut command);

    execute_command(&mut command);
}

#[cfg(not(debug_assertions))]
fn conditional_cargo_release_flags(command: &mut Command) {
    command.arg("--release");
}
#[cfg(debug_assertions)]
#[allow(unused_variables)]
fn conditional_cargo_release_flags(command: &mut Command) {

}

pub fn distribute_utensils(utensils_dir: &Path, app_dir: &Path) {
    println!("Distributing utensils for {}", app_dir.file_name().unwrap().to_str().unwrap());

    distribute_utensil_to_project_dir           (&utensils_dir, app_dir, "run_kernel", "launch");
    distribute_utensil_to_project_dir           (&utensils_dir, app_dir, "compiler", "compile");
    distribute_utensil_to_dependencies_dir      (&utensils_dir, app_dir, "compiler", "compile");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "compiler", "compile", "state");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "compiler", "compile", "processes");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "compiler", "compile", "schedules");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "compiler", "compile", "scheduler");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "compiler", "compile", "kernel");

    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "add_state", "add", "state");
    distribute_utensil_to_module_src_dir        (&utensils_dir, app_dir, "add_process", "add", "processes");
    distribute_utensil_to_projects_dir          (&utensils_dir, "add_project", "add");
}

fn distribute_utensil_to_projects_dir(utensils_dir: &Path,  tool_name: &str, tool_shortcut_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = worldsong_hierarchy::get_projects_dir().join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

fn distribute_utensil_to_project_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = app_dir.join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

fn distribute_utensil_to_dependencies_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = worldsong_hierarchy::get_dependencies_dir(app_dir).join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

fn distribute_utensil_to_module_src_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str, app_module_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = worldsong_hierarchy::get_module_src_dir(app_dir, app_module_name).join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

pub fn soft_link(origin: &Path, destination: &Path) {
    match fs::soft_link(origin, destination) {
        Ok(_)                           => (), //println!("    Created soft link between {} and {}", origin.display(), destination.display()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists    => (), //println!("    Soft link already exists between {} and {}, skipping.", origin.display(), destination.display()),
            _                           => println!("    Couldn't link {} and {}: {}", origin.display(), destination.display(), e),
        }
    }
}

// Below are link versions necessary for upcoming versions of rustc
/*
#[cfg(target_os = "linux")]
pub fn soft_link(origin: &Path, destination: &Path) {
    match std::os::unix::fs::symlink(origin, destination) {
        Ok(_)                           => (), //println!("    Created soft link between {} and {}", origin.display(), destination.display()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists    => (), //println!("    Soft link already exists between {} and {}, skipping.", origin.display(), destination.display()),
            _                           => println!("    Couldn't link {} and {}: {}", origin.display(), destination.display(), e),
        }
    }
}

#[cfg(target_os = "windows")]
pub fn soft_link(origin: &Path, destination: &Path) {
    match std::os::windows::fs::symlink_file(origin, destination) {
        Ok(_)                           => (), //println!("    Created soft link between {} and {}", origin.display(), destination.display()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists    => (), //println!("    Soft link already exists between {} and {}, skipping.", origin.display(), destination.display()),
            _                           => println!("    Couldn't link {} and {}: {}", origin.display(), destination.display(), e),
        }
    }
}
*/

pub fn to_snake_case(input: &str) -> String {
    let mut formatted = String::new();
    let mut first_letter = true;
    for character in input.chars() {
        if character.is_uppercase() && first_letter == false {
            formatted.push('_');
        }
        formatted.push(character.to_lowercase().next().unwrap());
        first_letter = false;
    }
    formatted
}

pub fn to_camel_case(input: &str) -> String {
    let mut formatted = String::new();
    let mut capitalize_next = false;
    let mut first_letter = true;
    for character in input.chars() {
        if character == '_' {
            capitalize_next = true;
            continue
        }
        if capitalize_next == true || first_letter == true {
            formatted.push(character.to_uppercase().next().unwrap());
        } else {
            formatted.push(character);
        }
        capitalize_next = false;
        first_letter = false;
    }
    formatted
}