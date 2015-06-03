//#![feature(unicode)]
//extern crate rustc_unicode;
extern crate worldsong_hierarchy;
extern crate worldsong_config;

//use rustc_unicode::str::UnicodeStr;
// use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::env::consts;
use std::fs;
//use std::fs::PathExt;
//use regex::Regex;

// use std::fs::File;


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

//TODO: Remove support for compiler config files, add back as worldsong_compiler flags.
/*
pub fn get_compile_config(command: &mut Command, current_dir: &Path, source_filename: &str, target_dir: &Path) -> String {

    let mut config_display = String::new();
    match File::open(&Path::new(&worldsong_hierarchy::get_compile_config(&current_dir))) {
        Err(_) => {
            command.args(&worldsong_config::get_default_rustc_flags());
        }
        Ok(ref mut file) => {
            let mut file_contents = String::new();
            match file.read_to_string(&mut file_contents) {
                Err(_) => {
                    command.args(&worldsong_config::get_default_rustc_flags());
                }
                Ok(_) => {
                    if file_contents.is_empty() /*|| file_contents.is_whitespace()*/ {
                        command.args(&worldsong_config::get_default_rustc_flags());
                    } else {
                        config_display.push_str("with \"");
                        for line in file_contents.lines_any() {
                            config_display.push_str(line);
                            config_display.push_str(" ");

                            for cmd in line.split(' ') {
                                command.arg(cmd);
                            }
                        }
                        config_display.push_str("\"");
                    }
                }
            }
        }
    }

    command.arg("--out-dir").arg(target_dir.as_os_str().to_str().unwrap());
    command.arg("-C").arg("prefer-dynamic");
    command.arg(source_filename);

    config_display
}
*/

pub fn extract_library_name_from_file_name(lib_path: &Path) -> String {
    lib_path.file_stem().unwrap().to_str().unwrap().split("-").next().unwrap().trim_left_matches(consts::DLL_PREFIX).to_string() // oh lordy
}

pub fn link_libraries(command: &mut Command, lib_dir: &Path) {
    command.arg("-L").arg(lib_dir);
    for entry in fs::read_dir(lib_dir).unwrap() {
        let lib_path = entry.unwrap().path();
        let name = extract_library_name_from_file_name(&lib_path);
        command.arg("--extern").arg(&format!("{}={}", name, lib_path.to_str().unwrap()));
    }
}

pub fn rustc_compile_bin(project_dir: &Path, dep_dirs: &Vec<PathBuf>, src_file_path: &Path, prefer_dynamic_linking: bool) {

    let module_name = src_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    println!("Building a binary for {}", module_name);
    let target_dir = &worldsong_hierarchy::get_module_target_dir(&project_dir, &module_name);
    fs::create_dir_all(target_dir).unwrap();

    let mut command = Command::new(worldsong_hierarchy::get_rustc_binary_path());

    for dir in dep_dirs.iter() {
        link_libraries(&mut command, dir)
    }

    if prefer_dynamic_linking {
        command.arg("-C").arg("prefer-dynamic");
    }

    command.arg("--out-dir").arg(target_dir);
    conditional_rustc_release_flags(&mut command);
    command.arg(src_file_path);


    execute_command(&mut command);
}

pub fn rustc_compile_lib(project_dir: &Path, dep_dirs: &Vec<PathBuf>, src_file_path: &Path, lib_type: &str, prefer_dynamic_linking: bool) {

    let module_name = src_file_path.parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();

    println!("Building a library for {}", module_name);
    let target_dir = &worldsong_hierarchy::get_module_target_dir(&project_dir, &module_name);
    fs::create_dir_all(target_dir).unwrap();

    let mut command = Command::new(worldsong_hierarchy::get_rustc_binary_path());

    for dir in dep_dirs.iter() {
        link_libraries(&mut command, dir)
    }

    command.arg(format!("--crate-type={}", lib_type));
    if prefer_dynamic_linking {
        command.arg("-C").arg("prefer-dynamic");
    }

    command.arg("--out-dir").arg(target_dir);
    conditional_rustc_release_flags(&mut command);
    command.arg(src_file_path);


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

/*
#[cfg(target_os = "linux")]
pub fn make_shortcut(origin: &Path, destination: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(origin, destination)
}

#[cfg(target_os = "windows")]
pub fn make_shortcut(origin: &Path, destination: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(origin, destination)
}*/