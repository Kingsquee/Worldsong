use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

use regex::Regex;
use worldsong_hierarchy;
use system;

/// Returns true if a process is created.
pub fn exec(app_dir: &Path, schedule_src_path: &Path) -> bool {

    let mut schedule_contents = String::new();
    File::open(schedule_src_path).unwrap().read_to_string(&mut schedule_contents).unwrap();

    struct Signiature {
        process_name: String,
        parameters: Vec<String>,
    }

    let mut schedule_process_sigs: Vec<Signiature> = Vec::new();

    // extract the process names and parameters via regex: <name>_process(<param>,<param>)
    // shove into schedule_process_sigs

    // parses function for name and parameter list
    let func_regex = Regex::new(r"([a-z,_]+)_process\((.+)+\)").unwrap();
    // seperates parameter list into individual parameters
    let param_regex = Regex::new(r"([a-z_]+|[^,\s,\(,\)])+").unwrap();

    // process: params

    for func_cap in func_regex.captures_iter(&schedule_contents) {
        let process_name = format!("{}{}", func_cap.at(1).unwrap(), "_process");
        let parameter_names = func_cap.at(2).unwrap().to_string();

        //println!("parameter names: {}", parameter_names);
        let mut parameters: Vec<String> = Vec::new();

        for param_cap in param_regex.captures_iter(&parameter_names) {
            for i in 1..param_cap.len() {
                let cap = param_cap.at(i).unwrap().to_string();
                //println!("Found parameter: {},", cap);
                parameters.push(cap);
            }
        }
        schedule_process_sigs.push(
            Signiature {
                process_name: process_name,
                parameters: parameters
            }
        );
    }

    let mut process_names: Vec<String> = Vec::new();

    for entry in fs::read_dir(worldsong_hierarchy::get_module_src_dir(app_dir, "processes")).unwrap() {
        let e = entry.unwrap().path();

        let metadata = fs::metadata(&e);
        if metadata.is_err() { continue };

        let extension = e.extension();
        if extension.is_none() { continue };

        let extension_str = e.extension().unwrap().to_str();
        if extension_str.is_none() || extension_str.unwrap() != "rs" { continue };

        let name = e.as_path().file_stem().unwrap().to_str().unwrap().to_string();
        process_names.push(name);
    }

    // if it finds a process name in the schedule that doesn't have a file_stem equivelent
    for sig in schedule_process_sigs.iter() {
        //println!("{}: ", sig.process_name);
        let mut found = false;
        for process_name in process_names.iter() {
            //println!("{} == {}? {}", process_name, &sig.process_name, process_name == &sig.process_name);
            if process_name == &sig.process_name {
                found = true;
                break;
            }
        }

        if found == false {
            // a call to a process name was found in the schedule, but the process doesn't exist.
            // create it.
            // have it call add_process with name --state "param1" --state "param2" --state "param3"
            let mut add_parameters = Vec::new();
            //add_parameters.push("--name");
            add_parameters.push(sig.process_name.as_ref());
            for parameter in sig.parameters.iter() {
                add_parameters.push("--state");
                add_parameters.push(parameter);
            }

            //testing
            add_parameters.push("--editor");
            add_parameters.push("kate");

            system::run(&worldsong_hierarchy::get_module_src_dir(app_dir, "processes").join(Path::new("add")), Some(add_parameters));
            println!("You entered an unrecognized process name: creating {}", &sig.process_name);
            return true
        }
    }

    return false

}