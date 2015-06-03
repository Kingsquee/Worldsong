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
    opts.optopt("e", "editor", "Open the state in the editor of choice.", "EDITOR");

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
        formatted = formatted.trim_right_matches("state").to_string();
        formatted = formatted.trim_right_matches("_").to_string();
        formatted

    } else {
        print_usage(&program, opts);
        return;
    };
    let editor = matches.opt_str("e");

    // Lets generate!
    let app_dir = worldsong_hierarchy::get_current_project_dir();

    // Create new dir
    let state_dir = worldsong_hierarchy::get_module_src_dir(&app_dir, "state");

    // Create the state's src file
    let state_src_file_name = name.clone() + "_state.rs";

    let state_src_path = state_dir.clone().join(state_src_file_name);
    let mut state_src_file = worldsong_hierarchy::create_file_all(&state_src_path).unwrap();

    let mut state_type_name = name.clone();
    let capital_first_letter = state_type_name.chars().next().unwrap().to_uppercase().next().unwrap();
    state_type_name.remove(0);
    state_type_name.insert(0, capital_first_letter);
    state_type_name.push_str("State");

    let state_src_text = format!(
"
data! (
    {name} {{
{indentation}
    }}
);", name = &state_type_name, indentation = r"        ");

    state_src_file.write_all(state_src_text.as_bytes()).unwrap();
    state_src_file.flush().unwrap();

    // Open the text file in editor of choice
    if editor.is_none() { return }
    let mut wraped_editor = match Editor::new(&editor.unwrap()) {
        Some(e) => e,
        None => panic!("Sorry, that editor isn't supported."),
    };

    wraped_editor.cursor(4,9);
    wraped_editor.open(&state_src_path);
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