use std::path::Path;

use worldsong_hierarchy;
use system;

pub fn exec(app_dir: &Path) {

    let cargo_toml_path = worldsong_hierarchy::get_dependencies_dir(app_dir);

    println!("Compiling {} dependencies", app_dir.file_stem().unwrap().to_str().unwrap());

    system::cargo_compile(&cargo_toml_path);
}