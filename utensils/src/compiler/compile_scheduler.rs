use std::path::Path;

use worldsong_hierarchy;
use worldsong_config;
use system;

pub fn exec(app_dir: &Path) {

    let module_name = "scheduler";

    let module_dir = worldsong_hierarchy::get_module_src_dir(app_dir, module_name);
    let src_file_path = module_dir.join(format!("{}.rs", module_name));

    //let config_display = system::get_compile_config(&mut command, &current_dir, &src_file_path, &target_dir);
    println!("Compiling {}", module_name/*, config_display*/);

    let mut dep_dirs = Vec::new();
    // Link dependencies
    for path in worldsong_hierarchy::get_dependencies_all_target_dirs(app_dir).iter() {
        dep_dirs.push(path.clone())
    }

    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "state"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "schedules"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "processes"));

    system::rustc_compile_lib(app_dir, &dep_dirs, &src_file_path, worldsong_config::get_scheduler_lib_type(), true);
}
