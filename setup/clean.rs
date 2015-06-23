extern crate worldsong_hierarchy;
extern crate walker;

use std::fs;
use std::env;
use walker::Walker;

fn main() {
    println!("Cleaning...");
    let wsroot = worldsong_hierarchy::get_worldsong_root_dir();

    let setup_target_dir_path = env::current_dir().unwrap().join("target");

    for entry in Walker::new(&wsroot).unwrap() {
        let path = entry.unwrap().path();
        //println!("Path: {:?}", path);
        let name = path.file_name().unwrap().to_str().unwrap();

        let metadata = match fs::metadata(&path) {
            Err(_) => continue,
            Ok(metadata) => metadata,
        };

        // println!("Path is {:?}", path);
        // println!("setup_target_dir_path is {:?}", setup_target_dir_path);
        // println!("Equal? {}", path == setup_target_dir_path);
        if path == setup_target_dir_path { continue }

        if metadata.is_dir() && name == "target" {
            fs::remove_dir_all(&path).unwrap();
            println!("    Removed target dir at {:?}", path);
        } else if metadata.is_file() && name == "Cargo.lock" {
            fs::remove_file(&path).unwrap();
            println!("    Removed Cargo.lock at {:?}", path);
        }
    }
    //panic!("debug");
}
