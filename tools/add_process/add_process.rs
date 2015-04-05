extern crate getopts;
extern crate wraped;
extern crate common;

use getopts::Options;

use std::fs;
use std::io::Write;

use std::env;
use std::fs::File;
use wraped::{Editor, EditorTrait};

use common::hierarchy;
use common::system;

fn main() {
    // Program args

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("n", "name", "Set the name of the process.", "NAME");
    opts.optopt("e", "editor", "Open the process in the editor of choice.", "EDITOR");

    let matches = match opts.parse(&args[1..]) {
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
    let mut process_dir = hierarchy::get_processes_dir();
    process_dir.push(name.clone());

    hierarchy::create_fresh_dir(&process_dir).unwrap();

    // Create the process's src file
    let process_src_file_name = name.clone() + "_process.rs";

    let process_src_path = process_dir.clone().join(process_src_file_name);
    let mut process_src_file = File::create(&process_src_path).unwrap();

    let process_src_text = String::from(
"extern crate state;
use state::{/*...*/};

pub fn execute(/*...*/) -> () {

}");

    process_src_file.write_all(process_src_text.as_bytes()).unwrap();
    process_src_file.flush().unwrap();

    // Copy the compile tool into the dir
    let compile_tool_path = hierarchy::get_compile_process_tool_target_dir().join("compile_process");
    match fs::copy(&compile_tool_path, &process_dir.join("compile")) {
        Ok(_) => (),
        Err(e) => {
            fs::remove_dir_all(&process_dir).unwrap();
            println!("Could not copy the compile tool to the directory, maybe try re-running your OS's setup tool?");
            panic!("{}", e)
        }
    }

    // Open the text file in editor of choice
    if editor.is_none() { return }
    let mut wraped_editor = match Editor::new(&editor.unwrap()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };

    wraped_editor.cursor(5,4);
    wraped_editor.open(&process_src_path);
    system::execute_command(&mut wraped_editor.get_command());
}
