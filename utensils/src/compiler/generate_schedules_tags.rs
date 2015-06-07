use worldsong_hierarchy;

use std::collections::{HashSet, HashMap};
use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

use regex::Regex;

// Generates tag files for each process, which define what schedule they're used in.
pub fn exec(app_dir: &Path) {
    // find schedule names
    let schedules_paths = worldsong_hierarchy::get_module_all_src_files(&app_dir, "schedules");

    // find processes
    let processes_paths = worldsong_hierarchy::get_module_all_src_files(&app_dir, "processes");

    // Process:[Schedule, Schedule, ...]
    let mut tags: HashMap<String, HashSet<&str>> = HashMap::with_capacity(processes_paths.len());

    // for each schedule,
        // parse the schedule for what processes are used
        // for each process
            // insert it into the hashmap's hashset
    for schedule_path in schedules_paths.iter() {
        let schedule_name = schedule_path.file_stem().unwrap().to_str().unwrap();

        for process_name in parse_schedule(schedule_path).iter() {

            if tags.contains_key(process_name) {
                tags.get_mut(process_name).unwrap().insert(schedule_name);
            } else {
                let mut tag_set = HashSet::with_capacity(schedules_paths.len());
                tag_set.insert(schedule_name);

                tags.insert(process_name.clone(), tag_set);
            }
        }
    }

    // for each process in the processes dir
        // if it's name is in the hashmap
            // write a tag file containing the schedule names the process belongs to
    for process_path in processes_paths.iter() {
        let process_name = process_path.file_stem().unwrap().to_str().unwrap();

        let mut tag_file = worldsong_hierarchy::create_file_all(&worldsong_hierarchy::get_file_tag_path(&app_dir, "schedules_tag", process_name)).unwrap();
        let schedule_tags_maybe = tags.get(process_path.file_stem().unwrap().to_str().unwrap());

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

// TODO: Give up on this --pretty bullshit and just regex the data! macro.
fn parse_schedule(schedule_path: &Path) -> Vec<String> {

    let mut process_names: Vec<String> = Vec::new();

    let mut schedule_src = String::new();
    File::open(schedule_path).unwrap().read_to_string(&mut schedule_src).unwrap();

    let re = match Regex::new(r"(?:(\w+)_process\(.*\))") {
        Ok(re) => re,
        Err(e) => panic!("{}", e),
    };

    for cap in re.captures_iter(&schedule_src) {
        match cap.at(1) {
            Some(name) => {
                //println!("Found: {}", name);
                process_names.push(name.to_string() + "_process")
            }
            None => {
                continue
            }
        }
    }
    process_names
}