use std::old_io::{File, Reader, Writer};
use std::old_path::{Path, GenericPath};

use common::hierarchy;

pub fn exec(struct_src_dirs: &Vec<Path>) {

    // Don't set is_compilng, since it's just generating the code to compile, not actually compiling it yet.

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
        let capital_first_letter: char = struct_type_name.char_at(0).to_uppercase().next().unwrap(); //wat
        struct_type_name.remove(0);
        struct_type_name.insert(0, capital_first_letter);
        struct_type_name.push_str("State");
        type_names.push(struct_type_name)
    }

    // Add the common lib import
    state_src_text.push_str(
"// ATTENTION: This file is automatically generated. Don't modify it unless your life is terrible.
#[macro_use]
extern crate common;\n\n"
    );

    state_src_text.push_str("// Now, lets get these imports out of the way...\n");

    for i in 0 .. names.len() {
        state_src_text.push_str(format!("pub use {}::{};\n", names[i].as_slice(), type_names[i].as_slice()).as_slice());
    }
    state_src_text.push_str("\n");

    let structs_dir_str = String::from_str(hierarchy::get_structs_dir().as_str().unwrap());
    for name in names.iter() {
        state_src_text.push_str(format!("#[path = \"{structs_dir}/{struct_name}/{struct_name}_state.rs\"]\n", structs_dir = structs_dir_str, struct_name = name.as_slice()).as_slice());
        state_src_text.push_str(format!("mod {};\n", name.as_slice()).as_slice());
    }
    state_src_text.push_str("\n\n");

    // Add a data! macro that lists $name: $NameState = $NameState::new() for each name
    state_src_text.push_str("data! {\n");
    state_src_text.push_str("    Data {\n");

    // TODO: Parse schedules and arrange by call relationships
    // for fewer cache misses
    for i in 0 .. names.len() {
        state_src_text.push_str(
            format!("       {name}: {type_name} = {type_name}::new()\n",
            name = names[i].as_slice(),
            type_name = type_names[i].as_slice(),
            ).as_slice()
        );
    }

    state_src_text.push_str("    }\n");
    state_src_text.push_str("}\n");

    let state_src_path = hierarchy::get_state_src_dir().join("state.rs");

    // save as state.rs
    println!("Creating new state.rs");
    let mut state_src_file = File::create(&state_src_path);
    state_src_file.write_str(state_src_text.as_slice()).unwrap();
    state_src_file.flush().unwrap();
}
