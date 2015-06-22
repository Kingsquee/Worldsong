use std::path::Path;

use worldsong_hierarchy;
use utensils_common;

pub fn exec(app_dir: &Path) {

    let module_name = "scheduler";
    let module_dir = worldsong_hierarchy::get_module_src_dir(app_dir, module_name);
    let src_file_path = module_dir.join(format!("{}.rs", module_name));

    // Link dependencies
    let mut dep_dirs = Vec::new();
    for path in worldsong_hierarchy::get_dependencies_all_target_dirs(app_dir).iter() {
        dep_dirs.push(path.clone())
    }

    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "state"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "schedules"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "processes"));

    let config_file_path = worldsong_hierarchy::get_module_compile_config_path(&module_dir);
    utensils_common::rustc_compile_lib(app_dir, &dep_dirs, &src_file_path, &config_file_path);}
