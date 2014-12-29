extern crate getopts;
extern crate collections;

use getopts::{optopt,optflag,getopts,OptGroup};

use std::io;
use std::os;
use std::io::fs::File;
use collections::str::StrExt;

#[path = "./../tool_settings.rs"]
mod tool_settings;

#[path = "./../tool_helpers.rs"]
mod tool_helpers;

#[path = "./../../../common/fs.rs"]
mod fs;

fn main() {
    // Program args

    let args: Vec<String> = os::args();
    let opts = &[
        optopt("n", "name", "Set the name of the data struct.", "NAME")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let name = match matches.opt_str("n") {
        Some(s) => { s }
        None => { "new".to_string() }
    };

    // Lets generate!

    // Create the struct's directory
    let struct_dir = fs::get_state_dir().join(name.clone());
    io::fs::mkdir(&struct_dir, io::USER_RWX).unwrap();

    // Create the struct's src file
    let struct_src_file_name = name.clone() + ".rs";

    let struct_src = struct_dir.clone().join(struct_src_file_name);
    let mut struct_src_file = File::create(&struct_src).unwrap();

    let mut struct_type_name = name.clone();
    let capital_first_letter = struct_type_name.char_at(0).to_uppercase();
    struct_type_name.remove(0);
    struct_type_name.insert(0, capital_first_letter);
    struct_type_name.push_str("State");

    let struct_src_text = format!(
"extern crate data_macro;
data! (
    {}: {} {{
        //go!
    }}
);", name.clone(), struct_type_name.clone());

    struct_src_file.write_str(struct_src_text.as_slice()).unwrap();
    struct_src_file.flush().unwrap();

    // Copy the compile script into the dir
    let compile_script_path = fs::get_compile_state_struct_script_target_dir().join("compile_state_struct");
    match io::fs::copy(&compile_script_path, &struct_dir.join("compile")) {
        Ok(_) => { () }
        Err(e) => {
            io::fs::rmdir_recursive(&struct_dir).unwrap();
            println!("Could not copy the compile script to the directory, maybe try re-running your OS's setup script?");
            panic!("{}", e)
        }
    }
}
