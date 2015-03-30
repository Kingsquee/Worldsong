#![feature(old_io)]
#![feature(old_path)]
#![feature(std_misc)]

extern crate state;
extern crate common;
extern crate time;

use std::dynamic_lib::DynamicLibrary;
use std::mem;
use std::old_io;
use std::old_io::File;
use std::old_path::Path;
use std::old_path::GenericPath;

use state::Data;
use std::time::duration::Duration;

use common::hierarchy;

fn main() {
    //let data_dylib_path = find_data_dylib().unwrap();
    let scheduler_dylib_path = find_scheduler_dylib().unwrap();

    //let mut data_dylib:             Option<DynamicLibrary>      = Some(load_library(data_dylib_path));
    let mut scheduler_dylib:        Option<DynamicLibrary>      = Some(load_library(&scheduler_dylib_path));

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
            let mut timer = old_io::Timer::new().unwrap();
            while File::open(&hierarchy::get_is_compiling_tag()).is_ok() {
                println!("Compilation is still ongoing. Trying again in 1 second...");
                timer.sleep(Duration::seconds(1));
            }

            // Load new library from disk
            scheduler_dylib         = Some(load_library(&scheduler_dylib_path));
            scheduler_run_symbol    = Some(load_scheduler_run_symbol(scheduler_dylib.as_ref().unwrap()));

            data.core.reload = false;
        }
        // TODO: Would be nice to have this load the latest state::Data from disk.
        else if data.core.reset {
            println!("Resetting state...");
            data = Data::new();

            data.core.reset = false;
        }
    }
}

/*
fn find_data_dylib() -> Option<Path> {
    // look in target dir
    let common_target_dir = common::fs::get_common_target_dir();
    // find the dylib
    let contents = fs::readdir(&common_target_dir).unwrap();
    for entry in contents.iter() {
        if entry.filename_str().unwrap().starts_with("libcommon") {
            return Some(entry.clone())
        }
    }
    None
}
*/

fn find_scheduler_dylib() -> Option<Path> {
    // look in target dir
    let scheduler_target_dir = hierarchy::get_scheduler_target_dir();
    // find the dylib
    let contents = old_io::fs::readdir(&scheduler_target_dir).unwrap();
    for entry in contents.iter() {
        if entry.filename_str().unwrap().starts_with("libscheduler") {
            return Some(entry.clone())
        }
    }
    None
}

fn load_library(path: &Path) -> DynamicLibrary {
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

/*
fn load_data_new_symbol(dylib: &DynamicLibrary) -> fn() -> Data {
    println!("Loading data new symbol");
    unsafe {
        match dylib.symbol::<fn() -> Data>("new") {
            Err (why)   => { panic! ("Data loading error: {}", why); }
            Ok  (func)  => { mem::transmute(func) }
        }
    }
}
*/
