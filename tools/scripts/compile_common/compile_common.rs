use std::os;
use std::io;
use std::io::fs;

#[path = "./../compile_settings.rs"]
mod compile_settings;

/// Compiles the common lib, and everything else, wot.
fn main() {

    // Lets compile!
    compile_settings::set_is_compiling(true);

    let current_dir = os::self_exe_path().unwrap();
    let target_path = current_dir.join("target");

    compile_settings::create_fresh_dir(&target_path);

    println!("Compiling the Common library");

    let mut compile_common_command = io::Command::new(compile_settings::get_cargo_path().as_str().unwrap());
    compile_common_command.arg("build");

    compile_settings::execute_command(compile_common_command);

    for path in compile_settings::get_all_process_src_dirs().iter_mut() {
        compile_settings::run_external_application(&path.join("compile"), Some(vec!["-c"]));
    }

    for path in compile_settings::get_all_schedule_src_dirs().iter_mut() {
        compile_settings::run_external_application(&path.join("compile"), Some(vec!["-c"]));
    }

    compile_settings::run_external_application(
        &compile_settings::get_scheduler_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    compile_settings::run_external_application(
        &compile_settings::get_kernel_src_dir().join("compile"),
        Some(vec!["-c"])
    );

    compile_settings::set_is_compiling(false);

}
