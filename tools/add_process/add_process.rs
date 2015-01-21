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
        optopt("n", "name", "Set the name of the process.", "NAME"),
        optopt("e", "editor", "Open the process in the editor of choice.", "EDITOR")
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
    let process_dir = os::self_exe_path().unwrap().join(name.clone());
    let filename = name.clone();
    hierarchy::create_fresh_dir(&process_dir).unwrap();

    // Create the process's src file
    let process_src_file_name = name.clone() + "_process.rs";

    let process_src_path = process_dir.clone().join(process_src_file_name);
    let mut process_src_file = File::create(&process_src_path).unwrap();

    let process_src_text = String::from_str(
"extern crate state;
use state::{/*...*/};

pub fn execute(/*...*/) -> () {
    
}");

    process_src_file.write_str(process_src_text.as_slice()).unwrap();
    process_src_file.flush().unwrap();

    // Copy the compile tool into the dir
    let compile_tool_path = hierarchy::get_compile_process_tool_target_dir().join("compile_process");
    match io::fs::copy(&compile_tool_path, &process_dir.join("compile")) {
        Ok(_) => (),
        Err(e) => {
            io::fs::rmdir_recursive(&process_dir).unwrap();
            println!("Could not copy the compile tool to the directory, maybe try re-running your OS's setup tool?");
            panic!("{}", e)
        }
    }

    // Open the text file in editor of choice
    if editor.is_none() { return }
    let mut wraped_editor = match Editor::new(editor.unwrap().as_slice()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };
    
    wraped_editor.cursor(5,4);
    wraped_editor.open(&process_src_path);
    system::execute_command(&mut wraped_editor.get_command());
}
