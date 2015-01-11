extern crate getopts;
extern crate toml;
extern crate common;

use getopts::{optopt,optflag,getopts,OptGroup};
use std::os;
use std::io;
use std::io::File;
use std::io::fs::PathExtensions

use common::hierarchy;
use common::system;
use common::settings;

// Generates the state library source file
fn main() {
    let struct_src_dirs: Vec<Path> = hierarchy::get_all_struct_src_dirs();
    
    // Don't set is_compilng, since it's just generating the code to compile, not actually compiling it yet.
    generate_cargo_toml(&struct_src_dirs);
    generate_src(&struct_src_dirs);
}

fn generate_cargo_toml(&struct_src_dirs: Vec<Path>) {
    println!("Generating the state library's cargo toml.");
    
    // Get the tomls
    let struct_dependency_tomls: Vec<Path> = Vec::with_capacity(struct_src_dirs.len());
    for dir in struct_src_dirs.iter() {
        let dependencies_toml = dir.clone().join("Dependencies.toml");
        
        if dependencies_toml.exists() && dependencies_toml.is_file() {
            struct_dependencies.push(dependencies_toml)
        }
    }
    
    struct Dependency {
        name: String,
        version: Option<String>,
    }
    
    // Extract the dependencies and versions from the tomls
    let struct_dependencies: Vec<Dependency> = Vec::with_capacity(struct_dependency_tomls.len());
    for toml_path in struct_dependency_tomls.iter() {
        let toml_file = match File::open(toml_path) {
            Ok(f) => f,
            Err(e) => {
                println!("Error: Could not open {}: {}", toml_path.filename_str().unwrap(), e);
                continue;
            }
        }
        let toml_text = match File::read_to_string() {
            Ok(f) => f,
            Err(e) => {
                println!("Error: Couldn't read {}: {}"), toml_path.filename_str().unwrap(), e);
                continue;
            }
        }
        
    }
    
    hierarchy::set_state_cargo_toml_needs_regen(false);
}

fn generate_src(&struct_src_dirs: Vec<Path>) {
    println!("Generating the state library's source code.");
    
    let mut state_src_text = String::new();
    
    // Get the struct folder names!
    let mut names: Vec<String> = Vec::new();
    for dir in struct_src_dirs.iter() {
        names.push(dir.filename_str().unwrap().to_string());
    }
    
    let mut type_names: Vec<String> = Vec::new();
    for name in names.iter() {
        let mut struct_type_name = name.clone();
        let capital_first_letter = struct_type_name.char_at(0).to_uppercase();
        struct_type_name.remove(0);
        struct_type_name.insert(0, capital_first_letter);
        struct_type_name.push_str("State");
        type_names.push(struct_type_name)
    }
    
    // Add the data_macro import
    state_src_text.push_str(
"#![feature(phase)]
#[phase(plugin)]
extern crate data_macro;\n\n"
    );
    
    // Add "extern crate $name" for each name
    for type_name in type_names.iter() {
        state_src_text.push_str(format!("extern crate {};\n", type_name.as_slice()).as_slice());
    }
    state_src_text.push_str("\n");
    
    // Add a data! macro that lists $name: $NameState = $NameState::new() for each name
    state_src_text.push_str("data! {\n");
    state_src_text.push_str("    Data {\n");
    
    // TODO: Parse schedules and arrange by call relationships
    // for fewer cache misses
    for i in range(0, names.len()) {
        state_src_text.push_str(
            format!("       {name}: {type_name} = {type_name}::new()\n", 
            name = names[i].as_slice(),
            type_name = type_names[i].as_slice()
            ).as_slice()
        );
    }
    
    state_src_text.push_str("    }\n");
    state_src_text.push_str("}\n");
    
    // save as state.rs
    let mut state_src_file = File::create(&hierarchy::get_state_src_dir().join("state.rs"));
    state_src_file.write_str(state_src_text.as_slice()).unwrap();
    state_src_file.flush().unwrap();
    
    hierarchy::set_state_src_needs_regen(false);
}
