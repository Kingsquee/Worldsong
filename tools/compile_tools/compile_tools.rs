extern crate getopts;
extern crate common;

use getopts::Options;
use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::process;

use common::hierarchy;
use common::system;

/// Compiles the kernel, duh.
fn main() {

    // Program args
    let mut compile_everything: bool = false;

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("a", "all", "Runs all compile tools after generating them.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("a") {
        compile_everything = true
    };

    let tools_dir = hierarchy::get_tools_dir();

    println!("Generating run tool for Kernel.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "run_kernel"));

    println!("Generating Cargo.toml tool for the State library.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "generate_state_library"));

    println!("Generating compilation tool for the State library.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_state_library"));

    println!("Generating compilation tool for the State Struct libraries");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_state_struct"));

    println!("Generating compilation tool for the Kernel.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_kernel"));

    println!("Generating compilation tool for the Scheduler.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_scheduler"));

    println!("Generating compilation tool for Schedules.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_schedule"));

    println!("Generating compilation tool for Processes.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "compile_process"));

    println!("Generating schedule tagging tool for Processes.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "generate_schedule_tags"));

    println!("Generating add tool for State Structs.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "add_state_struct"));

    println!("Generating add tool for Processes.");
    cargo_compile(&get_cargo_toml_path(&tools_dir, "add_process"));

    distribute_kernel_tool();
    distribute_generate_state_library_tool();
    distribute_compile_state_library_tool();
    distribute_compile_state_struct_tool();
    distribute_scheduler_tool();
    distribute_schedule_tools();
    distribute_process_tools();
    distribute_run_tool();
    distribute_add_state_struct_tool();
    distribute_add_process_tool();

    if compile_everything {
        compile_project();
    }
}

fn compile_project() {
    println!("Compiling the common library");
    cargo_compile(&hierarchy::get_common_src_dir().join("Cargo.toml"));

    system::run(&hierarchy::get_state_src_dir().join("generate"), None);
    system::run(&hierarchy::get_state_src_dir().join("compile"), None);

    system::run(&hierarchy::get_generate_schedule_tags_target_dir().join("generate_schedule_tags"), None);
}

fn get_cargo_toml_path(path: &Path, tool_name_str: &str) -> PathBuf {
    let tool_name = tool_name_str.to_string();
    path.join(tool_name.clone()).join("Cargo.toml")
}

fn distribute_generate_state_library_tool() {
    println!("Distributing generation tool for the State Library.");

    let file_origin = hierarchy::get_generate_state_library_tool_target_dir().join("generate_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("generate");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), &file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_library_tool() {
    println!("Distributing compile tool for the State Library.");

    let file_origin = hierarchy::get_compile_state_library_tool_target_dir().join("compile_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("compile");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_struct_tool() {
    println!("Distributing compilation tools for the State Structs.");

    let file_origin = hierarchy::get_compile_state_struct_tool_target_dir().join("compile_state_struct");

    let state_struct_dirs = hierarchy::get_all_struct_src_dirs();
    for dir in state_struct_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_kernel_tool() {
    println!("Distributing compilation tool for the Kernel.");

    let file_origin = hierarchy::get_compile_kernel_tool_target_dir().join("compile_kernel");
    let file_destination = hierarchy::get_kernel_src_dir().join("compile");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_scheduler_tool() {
    println!("Distributing compilation tool for the Scheduler.");

    let file_origin = hierarchy::get_compile_scheduler_tool_target_dir().join("compile_scheduler");
    let file_destination = hierarchy::get_scheduler_src_dir().join("compile");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_schedule_tools() {
    println!("Distributing compilation tools for the Schedules.");

    let file_origin = hierarchy::get_compile_schedule_tool_target_dir().join("compile_schedule");
    let schedules_src_dirs = hierarchy::get_all_schedule_src_dirs();

    for dir in schedules_src_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_process_tools() {
    println!("Distributing compilation tools for the Processes.");

    let file_origin = hierarchy::get_compile_process_tool_target_dir().join("compile_process");

    for dir in hierarchy::get_all_process_src_dirs().iter() {
        let file_destination = dir.clone().join("compile");
        match fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_run_tool() {
    println!("Distributing run tool for the Kernel.");

    let file_origin = hierarchy::get_run_kernel_tool_target_dir().join("run_kernel");
    let file_destination = hierarchy::get_worldsong_root_dir().join("launch");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_add_state_struct_tool() {
    println!("Distributing generation tool for State Structs.");

    let file_origin = hierarchy::get_add_state_struct_tool_target_dir().join("add_state_struct");
    let file_destination = hierarchy::get_structs_dir().join("add");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_add_process_tool() {
    println!("Distributing generation tool for Processes.");

    let file_origin = hierarchy::get_add_process_tool_target_dir().join("add_process");
    let file_destination = hierarchy::get_processes_dir().join("add");

    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.display(), file_destination.display()),
        Err(e) => println!("    {}", e),
    }
}

/*
fn compile(tool_filename: &Path) {

    let mut target_dir: PathBuf = tool_filename.to_path_buf();
    target_dir.pop();
    target_dir.push("target");

    hierarchy::create_fresh_dir(&target_dir);

    let mut command = process::Command::new(hierarchy::get_rustc_path().as_os_str());
    command.arg("--out-dir").arg(target_dir.as_os_str());
    command.arg(tool_filename.as_os_str());

    system::execute_command(&mut command);
}
*/

fn cargo_compile(cargo_toml_path: &Path) {

    let mut command = process::Command::new(hierarchy::get_cargo_path().as_os_str());
    command.arg("build");
    command.arg("--manifest-path");
    command.arg(cargo_toml_path.as_os_str());

    system::execute_command(&mut command);
}
