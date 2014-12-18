use std::io;

#[path = "./../tool_settings.rs"]
mod tool_settings;

#[path = "./../tool_helpers.rs"]
mod tool_helpers;

#[path = "./../../../common/fs.rs"]
mod fs;

/// Compiles the kernel, duh.
fn main() {

    //fs::set_system_paths();

    let scripts_dir = fs::get_compile_scripts_dir();

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
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_common");
    let file_destination = fs::get_common_src_dir().join("compile");
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_kernel_script() {
    println!("Distributing compilation script for the Kernel.");
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_kernel");
    let file_destination = fs::get_kernel_src_dir().join("compile");
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_scheduler_script() {
    println!("Distributing compilation script for the Scheduler.");
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_scheduler");
    let file_destination = fs::get_scheduler_src_dir().join("compile");
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn distribute_schedule_scripts() {
    println!("Distributing compilation scripts for the Schedules.");
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_schedule");

    let schedules_src_dirs = fs::get_all_schedule_src_dirs();

    for dir in schedules_src_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_process_scripts() {
    println!("Distributing compilation scripts for the Processes.");
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "compile_process");

    let process_dirs = fs::get_all_process_src_dirs();

    for dir in process_dirs.iter() {
        let file_destination = dir.clone().join("compile");
        match io::fs::copy(&file_origin, &file_destination) {
            Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
            Err(e) => println!("    {}", e),
        }
    }
}

fn distribute_run_script() {
    println!("Distributing run script for the Kernel.");
    let scripts_dir = fs::get_compile_scripts_dir();
    let file_origin = get_bin_path(&scripts_dir, "run_kernel");

    let file_destination = fs::get_worldsong_root_dir().join("launch");
    match io::fs::copy(&file_origin, &file_destination) {
        Ok(_) => println!("    Copied {} to {}", file_origin.filename_str().unwrap(), file_destination.as_str().unwrap()),
        Err(e) => println!("    {}", e),
    }
}

fn compile(tool_filename: Path) {

    let mut target_dir: Path = tool_filename.clone();
    target_dir.pop();
    target_dir.push("target");

    fs::create_fresh_dir(&target_dir);

    let mut command = io::Command::new(fs::get_rustc_path().as_str().unwrap());
    command.arg("--out-dir").arg(target_dir.as_str().unwrap());
    command.arg(tool_filename);

    tool_helpers::execute_command(command);
}
