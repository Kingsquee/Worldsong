use std::path::Path;
use std::fs;
use std::env::consts;
use worldsong_hierarchy;
use system;

pub fn exec(app_dir: &Path) {

    let module_dir = worldsong_hierarchy::get_module_src_dir(app_dir, "kernel");

    // Decide which kernel to build
    let mut src_file_path = module_dir.join("dynamic_kernel.rs");

    let scheduler_dylib_path = worldsong_hierarchy::get_module_target_dir(app_dir, "scheduler")
        .join(&format!("{}{}{}", consts::DLL_PREFIX, "scheduler", consts::DLL_SUFFIX));

    if fs::metadata(scheduler_dylib_path).is_err() {
        println!("Scheduler was not compiled as a dynamic library: Compiling static kernel.");
        src_file_path = module_dir.join("static_kernel.rs");
    }

    // Link dependencies
    let mut dep_dirs = Vec::new();
    for path in worldsong_hierarchy::get_dependencies_all_target_dirs(app_dir).iter() {
        dep_dirs.push(path.clone())
    }

    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "state"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "scheduler"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "schedules"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "processes"));

    // Get config file
    let mut config_path = worldsong_hierarchy::get_file_compile_config_path(&src_file_path);

    if fs::metadata(&config_path).is_err() {
        config_path = worldsong_hierarchy::get_module_compile_config_path(&module_dir);
    } else {
        println!("Using {} override.", config_path.file_name().unwrap().to_str().unwrap());
    }

    system::rustc_compile_bin(app_dir, &dep_dirs, &src_file_path, &config_path);
}