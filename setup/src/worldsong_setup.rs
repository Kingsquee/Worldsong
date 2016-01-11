extern crate worldsong_hierarchy;
extern crate utensils_common;

use std::path::{PathBuf, Path};

fn main() {
    let utensils_dir = worldsong_hierarchy::get_worldsong_root_dir().join("utensils");

    let app_dirs = worldsong_hierarchy::get_all_project_dirs();

    println!("Compiling {} dependencies", utensils_dir.file_name().unwrap().to_str().unwrap() );
    utensils_common::cargo_compile(&worldsong_hierarchy::get_dependencies_dir(&utensils_dir));

    //get depot deps dir
    let deps_dirs = worldsong_hierarchy::get_dependencies_all_target_dirs(&utensils_dir);

    rustc_compile_bin(&utensils_dir, &deps_dirs, "compiler");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "add_process");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "add_state");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "add_project");
    rustc_compile_bin(&utensils_dir, &deps_dirs, "run_kernel");

    for app_dir in app_dirs.iter() {
        utensils_common::distribute_utensils(&utensils_dir, app_dir);
    }

    for app_dir in app_dirs.iter() {
        let app_name = app_dir.file_name().unwrap();
        println!("Building {:?}", app_name);

        // Compiling dependencies will force a recompile of the whole project
        utensils_common::run(&worldsong_hierarchy::get_dependencies_dir(&app_dir).join("compile"), None);
    }
}

fn rustc_compile_bin(utensils_dir: &Path, deps_dirs: &Vec<PathBuf>, module_name: &str){
    let module_dir = worldsong_hierarchy::get_module_src_dir(&utensils_dir, module_name);
    let src_path = module_dir.join(&format!("{}.rs", module_name));
    worldsong_hierarchy::create_fresh_dir(&worldsong_hierarchy::get_module_target_dir(&utensils_dir, module_name)).unwrap();


    let config_file_path = worldsong_hierarchy::get_module_compile_config_path(&module_dir);
    utensils_common::rustc_compile_bin(utensils_dir, deps_dirs, &src_path, &config_file_path);

}
