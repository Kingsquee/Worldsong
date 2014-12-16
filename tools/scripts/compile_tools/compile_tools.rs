use std::io;
use std::io::fs;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the kernel, duh.
fn main() {

    //compile_settings::set_system_paths();

    let scripts_dir = compile_settings::get_compile_scripts_dir();

    println!("Generating run script for Kernel.");
    compile(get_src_path(&scripts_dir, "run_kernel"));

    println!("Generating compilation script for the Common library.");
    compile(get_src_path(&scripts_dir, "compile_common"));

    println!("Generating compilation script for the Kernel.");
    compile(get_src_path(&scripts_dir, "compile_kernel"));

    println!("Generating compilation script for the Scheduler.");
    compile(get_src_path(&scripts_dir, "compile_scheduler"));

    println!("Generating compilation script for Schedules.");
    compile(get_src_path(&scripts_dir, "compile_schedule"));

    println!("Generating compilation script for Processes.");
    compile(get_src_path(&scripts_dir, "compile_process"));

    distribute_common_script();
    println!(" ");
    distribute_kernel_script();
    println!(" ");
    distribute_scheduler_script();
    println!(" ");
    distribute_schedule_scripts();
    println!(" ");
    distribute_process_scripts();
    println!(" ");
    distribute_run_script();
    println!(" ");
}

fn get_src_path(path: &Path, script_name_str: &str) -> Path {
    let script_name = script_name_str.to_string();
    path.join(script_name.clone()).join(script_name.clone() + ".rs")
}

fn get_bin_path(path: &Path, script_name_str: &str) -> Path {
    let script_name = script_name_str.to_string();
    path.join(script_name.clone()).join("target").join(script_name.clone())
}

fn distribute_common_script() {
    println!("Distributing compilation script for the Common library.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_common");
    let file_destination = compile_settings::get_common_src_dir().join("compile");
    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_kernel_script() {
    println!("Distributing compilation script for the Kernel.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_kernel");
    let file_destination = compile_settings::get_kernel_src_dir().join("compile");
    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_scheduler_script() {
    println!("Distributing compilation script for the Scheduler.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_scheduler");
    let file_destination = compile_settings::get_scheduler_src_dir().join("compile");
    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_schedule_scripts() {
    println!("Distributing compilation scripts for the Schedules.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_schedule");

    let schedules_src_dirs = compile_settings::get_all_schedule_src_dirs();

    for dir in schedules_src_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_process_scripts() {
    println!("Distributing compilation scripts for the Processes.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_process");

    let process_dirs = compile_settings::get_all_process_src_dirs();

    for dir in process_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_run_script() {
    println!("Distributing run script for the Kernel.");
    let scripts_dir = compile_settings::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "run_kernel");

    let file_destination = compile_settings::get_worldsong_root_dir().join("launch");
    match fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn compile(tool_filename: Path) {

    let mut target_dir: Path = tool_filename.clone();
    target_dir.pop();
    target_dir.push("target");

    compile_settings::create_fresh_dir(&target_dir);

    let mut command = io::Command::new(compile_settings::get_rustc_path().as_str().unwrap());
    command.arg("--out-dir").arg(target_dir.as_str().unwrap());
    command.arg(tool_filename);

    compile_settings::execute_command(command);
}
