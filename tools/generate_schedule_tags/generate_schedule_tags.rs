extern crate common;
extern crate regex;


use std::collections::{HashSet, HashMap};
use std::io::Write;
use std::process::Command;
use std::path::Path;

use common::hierarchy;
use common::system;
//use std::str::StrExt;

use regex::Regex;

// Generates tag files for each process, which define what schedule they're used in.
fn main() {

    // find schedule names
    let schedule_dirs = hierarchy::get_all_schedule_src_dirs();

    // find processes
    let process_dirs = hierarchy::get_all_process_src_dirs();

    // Process:[Schedule, Schedule, ...]
    let mut tags: HashMap<String, HashSet<&str>> = HashMap::with_capacity(process_dirs.len());

    // for each schedule,
        // parse the schedule for what processes are used
        // for each process
            // insert it into the hashmap's hashset
    for dir in schedule_dirs.iter() {
        let schedule_name = dir.file_name().unwrap().to_str().unwrap();

        let mut schedule_filename = schedule_name.to_string();
        schedule_filename.push_str(".rs");

        for process_name in parse_schedule(&dir.join(schedule_filename)).iter() {

            if tags.contains_key(process_name) {
                tags.get_mut(process_name).unwrap().insert(schedule_name);
            } else {
                let mut tag_set = HashSet::with_capacity(schedule_dirs.len());
                tag_set.insert(schedule_name);

                tags.insert(process_name.clone(), tag_set);
            }
        }
    }

    // for each dir in the processes dir
        // if it's name is in the hashmap
            // write a tag file containing the schedule_names contents
    for process_dir in process_dirs.iter() {
        let mut tag_file = hierarchy::create_fresh_file(&hierarchy::get_schedule_tags(process_dir)).unwrap();
        let schedule_tags_maybe = tags.get(process_dir.file_name().unwrap().to_str().unwrap());

        if schedule_tags_maybe.is_some() {
            let schedule_tags = schedule_tags_maybe.unwrap();

            for tag in schedule_tags.iter() {
                tag_file.write_all(tag.as_bytes()).unwrap();
                tag_file.write_all(b"\n").unwrap();
            }
            tag_file.flush().unwrap();
        }
    }
}

fn parse_schedule(schedule_path: &Path) -> Vec<String> {

    let mut process_names: Vec<String> = Vec::new();

    let mut command = Command::new("rustc");
    command.arg("-Z").arg("unstable-options");
    command.arg("--pretty").arg("expanded");

    // Link dependencies dirs
    for path in hierarchy::get_state_dependency_dirs().iter() {
        system::link_libraries(&mut command, path);
    }

    // Link state
    system::link_libraries(&mut command, &hierarchy::get_state_target_dir());

    // Link process target dirs
    for process_target_dir in hierarchy::get_all_process_target_dirs().iter() {
        system::link_libraries(&mut command, process_target_dir);
    }

    // Link dependencies dirs
    for path in hierarchy::get_state_dependency_dirs().iter() {
        system::link_libraries(&mut command, path);
    }

    command.arg(schedule_path.as_os_str());

    let output = command.output().unwrap();
    let expanded_src = String::from_utf8_lossy(&output.stdout);

    //println!("{}", expanded_src);

    let re = match Regex::new(r"(\w+)_process::execute") {
        Ok(re) => re,
        Err(e) => panic!("{}", e),
    };

    for cap in re.captures_iter(&expanded_src) {
        match cap.at(1) {
            Some(name) => {
                //println!("Found: {}", name);
                process_names.push(name.to_string())
            }
            None => {
                continue
            }
        }
    }
    process_names
}
