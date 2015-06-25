extern crate worldsong_hierarchy;
use std::fs::File;
use std::io::Read;

pub struct VertexShader;

impl VertexShader {
    pub fn load(name: &str) -> String {
        load_text(&format!("{}.v.glsl", name))
    }
}

pub struct FragmentShader;

impl FragmentShader {
    pub fn load(name: &str) -> String {
        load_text(&format!("{}.f.glsl", name))
    }
}

fn load_text(filename: &str) -> String {
    let app_dir = worldsong_hierarchy::get_current_project_dir();
    let mut f = File::open(worldsong_hierarchy::get_module_src_dir(&app_dir, "shaders").join(filename)).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
