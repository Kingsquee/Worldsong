#![feature(std_misc)]
#![feature(start)]
extern crate worldsong_hierarchy;
extern crate state;
extern crate time;

use std::dynamic_lib::DynamicLibrary;
use std::mem;
use std::fs;
use std::thread;
use std::fs::File;
use std::path::{PathBuf, Path};
use std::env::consts;

use self::state::Data;

const RESET_STATE_STATUS_CODE: isize = 3;

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    let app_dir = worldsong_hierarchy::get_current_project_dir();
    let mut scheduler_dylib_path = find_scheduler_dylib(&app_dir).unwrap();

    let mut scheduler_dylib:        DynamicLibrary      = load_library(&scheduler_dylib_path);
    let mut scheduler_run_symbol:   fn(&mut Data) -> () = load_scheduler_run_symbol(&scheduler_dylib);

    let mut data = Data::new();

    loop {
        // Passing the hotloaded constructor to the hotloaded scheduler execution function.
        println!("Calling run");
        scheduler_run_symbol(&mut data);

        if data.core_state.quit {
            println!("Quitting.");
            break
        }
        else if data.core_state.reload {
            println!("Reloading scheduler...");

            // Drop all cached OS references
            drop(scheduler_dylib);
            drop(scheduler_run_symbol);

            // Check that compilation is finished
            while File::open(&worldsong_hierarchy::get_global_tag_path(&app_dir, "is_compiling")).is_ok() {
                println!("Compilation is still ongoing. Trying again in 1 second...");
                thread::sleep_ms(1000);
            }

            // Load new library from disk
            scheduler_dylib_path = find_scheduler_dylib(&app_dir).unwrap();
            scheduler_dylib         = load_library(&scheduler_dylib_path);
            scheduler_run_symbol    = load_scheduler_run_symbol(&scheduler_dylib);

            data.core_state.reload = false;
        }
        // TODO: Would be nice to have this load the latest state::Data from disk.
        else if data.core_state.reset {
            println!("Resetting state...");
            return RESET_STATE_STATUS_CODE
        }
    }

    return 0
}

fn find_scheduler_dylib(app_dir: &Path) -> Option<PathBuf> {
    // look in target dir
    let scheduler_target_dir = worldsong_hierarchy::get_module_target_dir(&app_dir, "scheduler");
    // find the dylib
    let contents = fs::read_dir(&scheduler_target_dir).unwrap();
    for entry in contents {
        let entry = entry.unwrap().path();
        if entry.file_name().unwrap().to_str().unwrap().starts_with(&format!("{}scheduler", consts::DLL_PREFIX)) {
            return Some(entry.clone())
        }
    }
    None
}

fn load_library(path: &Path) -> DynamicLibrary {
    println!("Loading library: {}", path.as_os_str().to_str().unwrap());
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