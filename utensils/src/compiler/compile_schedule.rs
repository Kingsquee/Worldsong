use std::path::Path;
use std::fs;

use worldsong_hierarchy;
use system;

pub fn exec(app_dir: &Path, src_file_path: &Path) {

    // Link dependencies
    let mut dep_dirs = Vec::new();
    for path in worldsong_hierarchy::get_dependencies_all_target_dirs(app_dir).iter() {
        dep_dirs.push(path.clone())
    }

    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "state"));
    dep_dirs.push(worldsong_hierarchy::get_module_target_dir(app_dir, "processes"));

    let mut config_path = worldsong_hierarchy::get_file_compile_config_path(src_file_path);

    if fs::metadata(&config_path).is_err() {
        let module_dir = src_file_path.parent().unwrap();
        config_path = worldsong_hierarchy::get_module_compile_config_path(&module_dir);
    } else {
        println!("Using {} override.", config_path.file_name().unwrap().to_str().unwrap());
    }

    system::rustc_compile_lib(app_dir, &dep_dirs, &src_file_path, &config_path);
}
