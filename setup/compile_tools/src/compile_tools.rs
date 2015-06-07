extern crate getopts;
extern crate worldsong_hierarchy;
extern crate system;

use getopts::Options;
use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::{PathBuf, Path};



/// Compiles the kernel, duh.
fn main() {

    // Program args
    let mut compile_everything: bool = false;

    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("a", "build-apps", "Builds the apps with the utensils.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("a") {
        compile_everything = true
    };

    let utensils_dir = worldsong_hierarchy::get_project_dir("utensils");

    // To build more apps, just add their directory names here.
    let app_dirs = worldsong_hierarchy::get_all_project_dirs();

    println!("Compiling {} dependencies", utensils_dir.file_name().unwrap().to_str().unwrap() );
    system::cargo_compile(&worldsong_hierarchy::get_dependencies_dir(&utensils_dir));

    //get depot deps dir
    let deps_dirs = worldsong_hierarchy::get_dependencies_all_target_dirs(&utensils_dir);

    rustc_compile_bin(&utensils_dir, &deps_dirs, "compiler");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "run_kernel");

    rustc_compile_bin(&utensils_dir, &deps_dirs, "add_state");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "add_process");

    for app_dir in app_dirs.iter() {
        println!("\nDistributing utensils for {}", app_dir.file_name().unwrap().to_str().unwrap());

        distribute_tool_to_project_dir          (&utensils_dir, app_dir, "run_kernel", "launch");
        distribute_tool_to_project_dir          (&utensils_dir, app_dir, "compiler", "compile");
        distribute_tool_to_dependencies_dir     (&utensils_dir, app_dir, "compiler", "compile");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "compiler", "compile", "state");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "compiler", "compile", "processes");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "compiler", "compile", "schedules");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "compiler", "compile", "scheduler");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "compiler", "compile", "kernel");

        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "add_state", "add", "state");
        distribute_tool_to_module_src_dir       (&utensils_dir, app_dir, "add_process", "add", "processes");
    }

    if !compile_everything { return }

    for app_dir in app_dirs.iter() {
        let app_name = app_dir.file_name().unwrap();
        println!("Building {:?}", app_name);

        // Compiling dependencies will force a recompile of the whole project
        system::run(&worldsong_hierarchy::get_dependencies_dir(&app_dir).join("compile"), None);
    }
}

fn rustc_compile_bin(utensils_dir: &Path, deps_dirs: &Vec<PathBuf>, module_name: &str){
    let module_dir = worldsong_hierarchy::get_module_src_dir(&utensils_dir, module_name);
    let src_path = module_dir.join(&format!("{}.rs", module_name));
    worldsong_hierarchy::create_fresh_dir(&worldsong_hierarchy::get_module_target_dir(&utensils_dir, module_name)).unwrap();


    let config_file_path = worldsong_hierarchy::get_module_compile_config_path(&module_dir);
    system::rustc_compile_bin(utensils_dir, deps_dirs, &src_path, &config_file_path);

}

fn distribute_tool_to_project_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = app_dir.join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

fn distribute_tool_to_dependencies_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = worldsong_hierarchy::get_dependencies_dir(app_dir).join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}

fn distribute_tool_to_module_src_dir(utensils_dir: &Path, app_dir: &Path, tool_name: &str, tool_shortcut_name: &str, app_module_name: &str) {
    let file_origin = worldsong_hierarchy::get_module_target_bin(utensils_dir, tool_name);
    let file_destination = worldsong_hierarchy::get_module_src_dir(app_dir, app_module_name).join(tool_shortcut_name);
    soft_link(&file_origin, &file_destination);
}


fn soft_link(source: &Path, dest: &Path) {
    match fs::soft_link(source, dest) {
        Ok(_)                           => println!("    Created soft link between {} and {}", source.display(), dest.display()),
        Err(e) => match e.kind() {
            ErrorKind::AlreadyExists    => println!("    Soft link already exists between {} and {}, skipping.", source.display(), dest.display()),
            _                           => println!("    Couldn't link {} and {}: {}", source.display(), dest.display(), e),
        }
    }
}