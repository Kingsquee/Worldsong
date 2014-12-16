extern crate common;
extern crate time;

use std::dynamic_lib::DynamicLibrary;
use std::mem;
use std::io;
use std::io::File;
use std::io::BufferedReader;
use std::io::fs::PathExtensions;

use common::data::Data;
use std::time::duration::Duration;

// I suppose there should be a common codebase for...maneuvering the project hierarchy...
// TODO: Move hierarchy functions of compile_settings.rs out to common/project_hierarchy.rs?
const DATA_PATH:        &'static str = "./../../common/target/libcommon-a91a912d114d0405.so";
const SCHEDULER_PATH:   &'static str = "./../../scheduler/target/libscheduler.so";

fn main() {
    //let mut data_dylib:             Option<DynamicLibrary>      = Some(load_library(Path::new(DATA_PATH)));
    let mut scheduler_dylib:        Option<DynamicLibrary>      = Some(load_library(Path::new(SCHEDULER_PATH)));

    //let mut data_new_symbol:        Option<fn() -> Data>        = Some(load_data_new_symbol(&data_dylib.unwrap()));
    let mut scheduler_run_symbol:   Option<fn(&mut Data) -> ()> = Some(load_scheduler_run_symbol(scheduler_dylib.as_ref().unwrap()));

    // TODO: hotloading data. Void pointer equivelent?
    let mut data = Data::new(); // = data_new_symbol();


    'main: loop {

        // Passing the hotloaded constructor to the hotloaded scheduler execution function.
        println!("Calling run");
        scheduler_run_symbol.unwrap()(&mut data);

        if data.core.quit {
            println!("Quitting.");
            break 'main
        }
        else if data.core.reload {
            println!("Reloading scheduler...");

            // Drop all cached OS references
            scheduler_dylib         = None;
            scheduler_run_symbol    = None;

            // Check that compilation is finished
            let mut timer = io::Timer::new().unwrap();
            while File::open(&get_worldsong_root_dir().join(".iscompiling")).is_ok() {
                println!("Compilation is still ongoing. Trying again in 1 second...");
                timer.sleep(Duration::seconds(1));
            }

            // Load new library from disk
            scheduler_dylib         = Some(load_library(Path::new(SCHEDULER_PATH)));
            scheduler_run_symbol    = Some(load_scheduler_run_symbol(scheduler_dylib.as_ref().unwrap()));

            data.core.reload = false;
        }
        else if data.core.reset {
            println!("Would be reloading data about now...");
            /*reset data*/
            data.core.reset = false;
        }
    }
}

fn load_library(path: Path) -> DynamicLibrary {
    println!("Loading library: {}", path.as_str().unwrap());
    match DynamicLibrary::open(Some(path)) {
        Err(why) => {
            panic!("Library loading error: {}", why);
        }
        Ok(binary) => {
            binary
        }
    }
}

fn load_scheduler_run_symbol(dylib: &DynamicLibrary) -> fn(&mut Data) -> () {
    println!("Loading scheduler run symbol");
    unsafe {
        match dylib.symbol::<fn(&mut Data) -> ()>("run") {
            Err (why)   => { panic! ("Scheduler loading error: {}", why); }
            Ok  (func)  => { mem::transmute(func) }
        }
    }
}

fn load_data_new_symbol(dylib: &DynamicLibrary) -> fn() -> Data {
    println!("Loading data new symbol");
    unsafe {
        match dylib.symbol::<fn() -> Data>("new") {
            Err (why)   => { panic! ("Data loading error: {}", why); }
            Ok  (func)  => { mem::transmute(func) }
        }
    }
}

//HACK: Will move compile_settings into commonlib, so this duplication is unncessary.
pub fn get_worldsong_root_dir() -> Path {
    let mut current_dir = std::os::self_exe_path().unwrap();

    loop {
        let contents = std::io::fs::readdir(&current_dir).unwrap();
        for entry in contents.iter() {
            if entry.is_file() && entry.filename_str().unwrap() == ".wsroot" {
                //println!("Worldsong root is {}", current_dir.display());
                return current_dir
            }
        }
        if !current_dir.pop() {
            panic!("ERROR: Could not find worldsong root. Was the .wsroot file removed?");
        }
    }
}
