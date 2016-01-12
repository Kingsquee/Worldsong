extern crate getopts;
extern crate wraped;
extern crate worldsong_hierarchy;
extern crate utensils_common;

use std::env;
use std::io::Write;
use wraped::{Editor, EditorTrait};
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} NAME [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Program args

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("e", "editor", "Open the process in the editor of choice.", "EDITOR");
    opts.optmulti("s", "state", "Adds a state parameter to the process", "STATE");
    opts.optflag("h", "help", "print this help menu"); 
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    
    if matches.free.is_empty() || matches.opt_present("h") {
        print_usage(&program, opts);
        return
    }

    let name = {

        let raw = matches.free[0].clone();
        let mut formatted = String::new();

        // they're using snake case
        if raw.contains("_") {
            // make sure they didn't use any capitals for some ungodly reason
            for character in raw.chars() {
                formatted.push(character.to_lowercase().next().unwrap());
            }
        } else { // they're using camel case
            // convert to snake case
            formatted = utensils_common::to_snake_case(&raw);
        }

        formatted.trim_right_matches(".rs")
                 .trim_right_matches("process")
                 .trim_right_matches("_").to_string()
    }; 
    
    let editor = matches.opt_str("e");

    let params = matches.opt_strs("s");

    // Lets-a go!
    let app_dir = worldsong_hierarchy::get_current_project_dir();

    // Create new dir
    let process_dir = worldsong_hierarchy::get_module_src_dir(&app_dir, "processes");

    // Create the process's src file
    let process_src_file_name = name.clone() + "_process.rs";

    let process_src_path = process_dir.clone().join(process_src_file_name);
    let mut process_src_file = worldsong_hierarchy::create_file_all(&process_src_path).unwrap();

    let mut formatted_imports = Vec::with_capacity(params.len());

    // generate imports
    for raw in params.iter() {
        // Format the import names as CamelCase
        let mut formatted_import = String::new();

        // they're using snake case
        if raw.contains("_") {
            formatted_import = utensils_common::to_camel_case(&raw);
        } else { // they're using camel case
            // make sure they didn't use underscores for some ungodly reason.
            for substring in raw.split('_') {
                formatted_import.push_str(&substring);
            }
            let capital_first_letter = formatted_import.chars().next().unwrap().to_uppercase().next().unwrap();
            formatted_import.remove(0);
            formatted_import.insert(0, capital_first_letter);
        }

        formatted_import = formatted_import.trim_right_matches(".rs").to_string();
        formatted_import = formatted_import.trim_right_matches("State").to_string();
        formatted_import = formatted_import.trim_right_matches("_").to_string();

        formatted_import.push_str("State");
        formatted_imports.push(formatted_import);
    }

    // generate parameters
    let mut formatted_params = Vec::with_capacity(params.len());
    for formatted_import in formatted_imports.iter() {
        // Format the parameter names as snake_case
        let mut formatted_parameter = utensils_common::to_snake_case(formatted_import);
        formatted_parameter.push_str(": &");
        formatted_parameter.push_str(&formatted_import);

        //println!("formatted_params: {:?}", &formatted_params);
        formatted_params.push(formatted_parameter)
    }

    // Remove duplicate imports
    formatted_imports.sort_by(|a, b| a.cmp(b));
    formatted_imports.dedup();

    //println!("{:?}", formatted_imports);

    // generate strings
    let mut formatted_imports_string = String::new();
    for i in 0..formatted_imports.len() {
        formatted_imports_string.push_str(&formatted_imports[i]);
        if i != formatted_imports.len()-1 {
            formatted_imports_string.push_str(", ");
        }
    }

    let mut formatted_params_string = String::new();
    for i in 0..formatted_params.len() {
        formatted_params_string.push_str(&formatted_params[i]);
        if i != formatted_params.len()-1 {
            formatted_params_string.push_str(", ");
        }
    }



    let process_src_text =
format!("extern crate state;
use state::{{{formatted_imports}}};

pub fn execute({formatted_params}) -> () {{
{indentation}
}}",    formatted_imports = formatted_imports_string, formatted_params = formatted_params_string,
        indentation = "    ");

    process_src_file.write_all(process_src_text.as_bytes()).unwrap();
    process_src_file.flush().unwrap();

    // Open the text file in editor of choice
    if editor.is_none() { return }
    let mut wraped_editor = match Editor::new(&editor.unwrap()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };

    wraped_editor.cursor(5,5);
    wraped_editor.open(&process_src_path);
    utensils_common::execute_command(&mut wraped_editor.get_command());
}
