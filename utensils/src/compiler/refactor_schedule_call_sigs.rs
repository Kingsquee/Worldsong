use std::fs::File;
use std::path::Path;
use regex::Regex;
use regex::NoExpand;
use std::io::{Read, Write};

use worldsong_hierarchy;

pub fn exec(app_dir: &Path, process_src_path: &Path) {

    // find the execution sig
    // get the parameter names as snake_case

    let mut process_src_file = File::open(&process_src_path).unwrap();
    let mut process_src = String::new();
    process_src_file.read_to_string(&mut process_src).unwrap();

    let execute_sig_regex = Regex::new(r"(?:pub fn execute[\s\r\n\t]*)\(([^\)]*)\)").unwrap();

    // seperates parameter list into individual state names

    let execute_params_regex = Regex::new(r"(&mut\s|&\s*)([A-Za-z]*)*").unwrap();
    let mut execute_params = Vec::new();


    let mut execute_sig = String::new();
    for cap in execute_sig_regex.captures_iter(&process_src) {
        execute_sig = cap.at(1).unwrap().to_string();
    }

    println!("execute_sig: {}", &execute_sig);

    for param_cap in execute_params_regex.captures_iter(&execute_sig) {
        //let reference_type = param_cap.at(1);
        let state_type = to_snake_case(&param_cap.at(2).unwrap());
        // store the reference_type and state_type (as snake_case) in a vec
        execute_params.push(state_type);
    }

    let process_name = process_src_path.file_stem().unwrap().to_str().unwrap();

    let mut new_call_sig = format!("{}(", process_name);

    for i in 0..execute_params.len() {
        new_call_sig.push_str(&execute_params[i]);
        if i == execute_params.len() -1 {
            new_call_sig.push(')');
        } else {
            new_call_sig.push_str(", ");
        }
    }

    println!("New call sig is {}", &new_call_sig);

    // regex-find the call sigs and parameters (as a group, not individually)
    // regex-replace the parameters with the snake_case'd parameter names

    let schedule_src_paths = worldsong_hierarchy::get_module_all_src_paths(app_dir, "schedules");

    let schedule_call_sig_regex = Regex::new(&format!("{}({})", process_name, r"\((.+)+\)")).unwrap();

    // TODO: Check schedule tags for what schedules to update.

    // open each schedule
    for schedule_src_path in schedule_src_paths.iter() {
        let mut schedule_src_file = File::open(schedule_src_path).unwrap();
        let mut schedule_src = String::new();
        schedule_src_file.read_to_string(&mut schedule_src).unwrap();

        schedule_src = schedule_call_sig_regex.replace_all(&schedule_src, NoExpand(&new_call_sig));

        //println!("schedule_call_sig_regex: {:?}", &schedule_call_sig_regex);
        //println!("{}", &schedule_src);

        schedule_src_file = File::create(schedule_src_path).unwrap();
        schedule_src_file.write_all(&schedule_src.as_bytes()).unwrap_or_else(|e| println!("Could not write to schedule: {:?}", e.kind()));
        schedule_src_file.sync_all().unwrap();
    }
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