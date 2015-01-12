extern crate getopts;
extern crate common;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;

use common::hierarchy;
use common::system;
use common::settings;

/// Compiles the kernel, duh.
fn main() {

    // Program args
    let mut compile_everything: bool = false;

    let args: Vec<String> = os::args();
    let opts = &[
        optflag("a", "all", "Runs all compile tools after generating them.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("a") {
        compile_everything = true
    };

    let tools_dir = hierarchy::get_tools_dir();

    println!("Generating run tool for Kernel.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "run_kernel"));
    
    println!("Generating Cargo.toml tool for the State library.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "generate_state_library"));

    println!("Generating compilation tool for the State library.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_state_library"));
    
    println!("Generating compilation tool for the State Struct libraries");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_state_struct"));

    println!("Generating compilation tool for the Kernel.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_kernel"));

    println!("Generating compilation tool for the Scheduler.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_scheduler"));

    println!("Generating compilation tool for Schedules.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_schedule"));

    println!("Generating compilation tool for Processes.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "compile_process"));
    
    println!("Generating add tool for State Structs.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "add_state_struct"));
    
    println!("Generating add tool for Processes.");
    cargo_compile(get_cargo_toml_path(&tools_dir, "add_process"));
    
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
    println!("Compiling the Common library");
    cargo_compile(hierarchy::get_common_src_dir().join("Cargo.toml"));

    system::run(&hierarchy::get_state_src_dir().join("generate"), None);
    system::run(&hierarchy::get_state_src_dir().join("compile"), None);
}

fn get_src_path(path: &Path, tool_name_str: &str) -> Path {
    let tool_name = tool_name_str.to_string();
    path.join(tool_name.clone()).join(tool_name.clone() + ".rs")
}

fn get_bin_path(path: &Path, tool_name_str: &str) -> Path {
    let tool_name = tool_name_str.to_string();
    path.join(tool_name.clone()).join("target").join(tool_name.clone())
}

fn get_cargo_toml_path(path: &Path, tool_name_str: &str) -> Path {
    let tool_name = tool_name_str.to_string();
    path.join(tool_name.clone()).join("Cargo.toml")
}

fn distribute_generate_state_library_tool() {
    println!("Distributing generation tool for the State Library.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "generate_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("generate");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_library_tool() {
    println!("Distributing compile tool for the State Library.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_struct_tool() {
    println!("Distributing compilation tools for the State Structs.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_state_struct");
    
    let state_struct_dirs = hierarchy::get_all_struct_src_dirs();
    for dir in state_struct_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_kernel_tool() {
    println!("Distributing compilation tool for the Kernel.");
    
    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_kernel");
    let file_destination = hierarchy::get_kernel_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_scheduler_tool() {
    println!("Distributing compilation tool for the Scheduler.");
    
    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_scheduler");
    let file_destination = hierarchy::get_scheduler_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_schedule_tools() {
    println!("Distributing compilation tools for the Schedules.");
    
    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_schedule");
    let schedules_src_dirs = hierarchy::get_all_schedule_src_dirs();

    for dir in schedules_src_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_process_tools() {
    println!("Distributing compilation tools for the Processes.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "compile_process");
    
    for dir in hierarchy::get_all_process_src_dirs().iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_run_tool() {
    println!("Distributing run tool for the Kernel.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "run_kernel");
    let file_destination = hierarchy::get_worldsong_root_dir().join("launch");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_add_state_struct_tool() {
    println!("Distributing generation tool for State Structs.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "add_state_struct");
    let file_destination = hierarchy::get_structs_dir().join("add");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_add_process_tool() {
    println!("Distributing generation tool for Processes.");

    let file_origin = get_bin_path(&hierarchy::get_tools_dir(), "add_process");
    let file_destination = hierarchy::get_processes_dir().join("add");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn compile(tool_filename: Path) {

    let mut target_dir: Path = tool_filename.clone();
    target_dir.pop();
    target_dir.push("target");

    hierarchy::create_fresh_dir(&target_dir);

    let mut command = io::Command::new(hierarchy::get_rustc_path().as_str().unwrap());
    command.arg("--out-dir").arg(target_dir.as_str().unwrap());
    command.arg(tool_filename);

    system::execute_command(&mut command);
}

fn cargo_compile(cargo_toml_path: Path) {

    let mut command = io::Command::new(hierarchy::get_cargo_path().as_str().unwrap());
    command.arg("build");
    command.arg("--manifest-path");
    command.arg(cargo_toml_path);

    system::execute_command(&mut command);
}
