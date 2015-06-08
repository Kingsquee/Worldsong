extern crate getopts;
extern crate wraped;
extern crate worldsong_hierarchy;
extern crate system;

use std::env;
use std::io::Write;
use wraped::{Editor, EditorTrait};
use getopts::Options;



fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Program args

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optopt("e", "editor", "Open the process in the editor of choice.", "EDITOR");
    opts.optmulti("s", "state", "Adds a state parameter to the process", "STATE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let name = if !matches.free.is_empty() {

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
            formatted = to_snake_case(&raw);
        }

        formatted = formatted.trim_right_matches(".rs").to_string();
        formatted = formatted.trim_right_matches("process").to_string();
        formatted = formatted.trim_right_matches("_").to_string();
        formatted

    } else {
        print_usage(&program, opts);
        return;
    };
    let editor = matches.opt_str("e");

    let params = matches.opt_strs("s");

    // Lets generate!
    let app_dir = worldsong_hierarchy::get_current_project_dir();

    // Create new dir
    let process_dir = worldsong_hierarchy::get_module_src_dir(&app_dir, "processes");

    // Create the process's src file
    let process_src_file_name = name.clone() + "_process.rs";

    let process_src_path = process_dir.clone().join(process_src_file_name);
    let mut process_src_file = worldsong_hierarchy::create_file_all(&process_src_path).unwrap();


    let mut formatted_imports = "".to_string();
    let mut formatted_params = "".to_string();
    for i in 0..params.len() {
        let raw = params[i].clone();

        // Format the import names as CamelCase
        let mut formatted_import = String::new();

        // they're using snake case
        if raw.contains("_") {
            formatted_import = to_camel_case(&raw);
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

        if i != params.len()-1 {
            formatted_import.push_str(", ");
        }

        // Format the parameter names as snake_case
        let mut formatted_parameter = String::new();

        // they're using snake case
        if raw.contains("_") {
            // make sure they didn't use any capitals for some ungodly reason
            for character in raw.chars() {
                formatted_parameter.push(character.to_lowercase().next().unwrap());
            }
        } else { // they're using camel case
            // convert to snake case
            formatted_parameter = to_snake_case(&raw);
        }

        formatted_parameter = formatted_parameter.trim_right_matches(".rs").to_string();
        formatted_parameter = formatted_parameter.trim_right_matches("state").to_string();
        formatted_parameter = formatted_parameter.trim_right_matches("_").to_string();

        formatted_parameter.push_str("_state");

        // Push formatted import
        formatted_imports.push_str(&formatted_import);

        // Push formatted parameter with the imported type
        formatted_params.push_str(&formatted_parameter);
        formatted_params.push_str(": &");
        formatted_params.push_str(&formatted_import);

        if i != params.len()-1 {
            formatted_params.push_str(", ");
        }
    }


    let process_src_text =
format!("extern crate state;
use state::{{{formatted_imports}}};

pub fn execute({formatted_params}) -> () {{
{indentation}
}}",    formatted_imports = formatted_imports, formatted_params = formatted_params,
        indentation = "    ");

    process_src_file.write_all(process_src_text.as_bytes()).unwrap();
    process_src_file.flush().unwrap();

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

fn to_snake_case(input: &str) -> String {
    let mut formatted = String::new();
    let mut first_letter = true;
    for character in input.chars() {
        if character.is_uppercase() && first_letter == false {
            formatted.push('_');
        }
        formatted.push(character.to_lowercase().next().unwrap());
        first_letter = false;
    }
    formatted
}

fn to_camel_case(input: &str) -> String {
    let mut formatted = String::new();
    let mut capitalize_next = false;
    let mut first_letter = true;
    for character in input.chars() {
        if character == '_' {
            capitalize_next = true;
            continue
        }
        if capitalize_next == true || first_letter == true {
            formatted.push(character.to_uppercase().next().unwrap());
        } else {
            formatted.push(character);
        }
        capitalize_next = false;
        first_letter = false;
    }
    formatted
}