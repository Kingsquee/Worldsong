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
        optflag("a", "all", "Runs all compile builders after generating them.")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("a") {
        compile_everything = true
    };

    let builders_dir = hierarchy::get_compile_builders_dir();

    println!("Generating run builder for Kernel.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "run_kernel"));
    
    println!("Generating Cargo.toml builder for the State library.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "generate_state_library"));

    println!("Generating compilation builder for the State library.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_state_library"));
    
    println!("Generating compilation builder for the State Struct libraries");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_state_struct"));

    println!("Generating compilation builder for the Kernel.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_kernel"));

    println!("Generating compilation builder for the Scheduler.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_scheduler"));

    println!("Generating compilation builder for Schedules.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_schedule"));

    println!("Generating compilation builder for Processes.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "compile_process"));
    
    println!("Generating add builder for State Structs.");
    cargo_compile(get_cargo_toml_path(&builders_dir, "add_state_struct"));
    
    //println!("Generating add builder for Processes.");

    distribute_kernel_builder();
    println!(" ");
    distribute_generate_state_library_builder();
    println!(" ");
    distribute_compile_state_library_builder();
    println!(" ");
    distribute_compile_state_struct_builder();
    println!(" ");
    distribute_scheduler_builder();
    println!(" ");
    distribute_schedule_builders();
    println!(" ");
    distribute_process_builders();
    println!(" ");
    distribute_run_builder();
    println!(" ");
    distribute_add_state_struct_builder();
    println!(" ");
    if compile_everything {
        //run_common_builder();
    }
}

fn get_src_path(path: &Path, builder_name_str: &str) -> Path {
    let builder_name = builder_name_str.to_string();
    path.join(builder_name.clone()).join(builder_name.clone() + ".rs")
}

fn get_bin_path(path: &Path, builder_name_str: &str) -> Path {
    let builder_name = builder_name_str.to_string();
    path.join(builder_name.clone()).join("target").join(builder_name.clone())
}

fn get_cargo_toml_path(path: &Path, builder_name_str: &str) -> Path {
    let builder_name = builder_name_str.to_string();
    path.join(builder_name.clone()).join("Cargo.toml")
}

fn run_common_builder() {
    let common_compile_builder = hierarchy::get_common_src_dir().join("compile");

    let mut command = io::Command::new(common_compile_builder.as_str().unwrap());
    command.cwd(&hierarchy::get_common_src_dir());

    system::execute_command(&mut command);
}

fn distribute_generate_state_library_builder() {
    println!("Distributing generation builder for the State Library.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "generate_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("generate");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_library_builder() {
    println!("Distributing compile builder for the State Library.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_state_library");
    let file_destination = hierarchy::get_state_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_compile_state_struct_builder() {
    println!("Distributing compilation builders for the State Structs.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_state_struct");
    
    let state_struct_dirs = hierarchy::get_all_struct_src_dirs();
    for dir in state_struct_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_kernel_builder() {
    println!("Distributing compilation builder for the Kernel.");
    
    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_kernel");
    let file_destination = hierarchy::get_kernel_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_scheduler_builder() {
    println!("Distributing compilation builder for the Scheduler.");
    
    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_scheduler");
    let file_destination = hierarchy::get_scheduler_src_dir().join("compile");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_schedule_builders() {
    println!("Distributing compilation builders for the Schedules.");
    
    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_schedule");
    let schedules_src_dirs = hierarchy::get_all_schedule_src_dirs();

    for dir in schedules_src_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_process_builders() {
    println!("Distributing compilation builders for the Processes.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "compile_process");
    
    for dir in hierarchy::get_all_process_src_dirs().iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_run_builder() {
    println!("Distributing run builder for the Kernel.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "run_kernel");
    let file_destination = hierarchy::get_worldsong_root_dir().join("launch");
    
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_add_state_struct_builder() {
    println!("Distributing generation builder for State Structs.");

    let file_origin = get_bin_path(&hierarchy::get_compile_builders_dir(), "add_state_struct");
    let file_destination = hierarchy::get_structs_dir().join("add");
    
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
