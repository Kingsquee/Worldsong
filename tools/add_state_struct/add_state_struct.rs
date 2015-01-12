extern crate getopts;
extern crate wraped;
extern crate collections;
extern crate common;

use getopts::{optopt,optflag,getopts,OptGroup};

use std::io;
use std::os;
use std::io::fs::File;
use collections::str::StrExt;
use wraped::{Editor, EditorTrait};

use common::hierarchy;
use common::system;
use common::settings;

fn main() {
    // Program args

    let args: Vec<String> = os::args();
    let opts = &[
        optopt("n", "name", "Set the name of the state struct.", "NAME"),
        optopt("e", "editor", "Open the state struct in the editor of choice.", "EDITOR")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let name = match matches.opt_str("n") {
        Some(s) => { s }
        None => { "new".to_string() }
    };

    let editor = match matches.opt_str("e") {
        Some(s) => { Some(s) }
        None => { None }
    };

    // Lets generate!

    // Create new dir
    let struct_dir = os::self_exe_path().unwrap().join(name.clone());
    let filename = name.clone();
    hierarchy::create_fresh_dir(&struct_dir).unwrap();

    // Create the Dependencies.toml
    let dependencies_toml_path = struct_dir.clone().join("Dependencies.toml");
    let mut dependencies_toml_file = File::create(&dependencies_toml_path).unwrap();
    let dependencies_toml_text = format!(
"[dependencies.data_macro]
git = \"https://github.com/Kingsquee/data-macro-rs\"
");

    dependencies_toml_file.write_str(dependencies_toml_text.as_slice()).unwrap();
    dependencies_toml_file.flush().unwrap();

    // Create the struct's src file
    let struct_src_file_name = name.clone() + "_state.rs";

    let struct_src_path = struct_dir.clone().join(struct_src_file_name);
    let mut struct_src_file = File::create(&struct_src_path).unwrap();

    let mut struct_type_name = name.clone();
    let capital_first_letter = struct_type_name.char_at(0).to_uppercase();
    struct_type_name.remove(0);
    struct_type_name.insert(0, capital_first_letter);
    struct_type_name.push_str("State");

    let struct_src_text = format!(
"
data! (
    {} {{
        
    }}
);", struct_type_name.clone());

    struct_src_file.write_str(struct_src_text.as_slice()).unwrap();
    struct_src_file.flush().unwrap();

    // Copy the compile tool into the dir
    let compile_tool_path = hierarchy::get_compile_state_struct_tool_target_dir().join("compile_state_struct");
    match io::fs::copy(&compile_tool_path, &struct_dir.join("compile")) {
        Ok(_) => (),
        Err(e) => {
            io::fs::rmdir_recursive(&struct_dir).unwrap();
            println!("Could not copy the compile tool to the directory, maybe try re-running your OS's setup tool?");
            panic!("{}", e)
        }
    }
    
    // set the tags that state needs regen
    hierarchy::set_state_src_needs_regen(true);
    hierarchy::set_state_cargo_toml_needs_regen(true);

    // Open the text file in editor of choice
    if editor.is_none() { return }
    let mut wraped_editor = match Editor::new(editor.unwrap().as_slice()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };
    
    wraped_editor.cursor(4,8);
    wraped_editor.open(&struct_src_path);
    system::execute_command(&mut wraped_editor.get_command());
}
