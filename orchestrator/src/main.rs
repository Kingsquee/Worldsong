extern crate time;
extern crate native;

extern crate dataset;

use time::precise_time_ns;

use std::mem;
use std::io::fs::PathExtensions;
use std::io::File;
use std::io::BufferedReader;
use std::dynamic_lib::DynamicLibrary;

use dataset::DB;

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    let mut app = Application::new();
    app.exec();
}

struct Application {
    update_cycle_symbols        :   Vec<fn() -> ()>,
    update_frame_symbols        :   Vec<fn(&mut DB) -> ()>,

    update_process_binaries     :   Vec<DynamicLibrary>,
    dataset                     :   DB,
    
    update_melody_path          :   Path,
    process_binaries_path       :   Path,
}

impl Application {

    pub fn new() -> Application {
        let mut app = Application {
            dataset                     :   DB::new(),
            
            update_cycle_symbols        :   Vec::new(),
            update_frame_symbols        :   Vec::new(),
            
            update_process_binaries     :   Vec::new(),
            
            update_melody_path          :   Path::new("./../../processes/update.melody"),
            process_binaries_path       :   Path::new("./../../processes/targets"),
        };
        app.hotload_processes();
        dataset::initialize(&app.dataset);
        app
    }
    
    // TODO: Will the u64 overflow make this screw up?
    fn exec(&mut self) {       
        let     max_fps     : u64 = 1;//60;
        let     skip_cycles : u64 = 1000000000 / max_fps;
        let mut next_cycle  : u64 = precise_time_ns();
        
        loop {
            if self.dataset.stop_execution { break }

            for func in self.update_cycle_symbols.iter() {
                (*func)()
            }
            
            if precise_time_ns() >= next_cycle {
                next_cycle += skip_cycles;
                
                if self.dataset.reload_processes == true {
                    self.dataset.halt_execution = false;
                    self.hotload_processes();
                    self.dataset.reload_processes = false;
                }
                
                //DEBUG: hotload every frame, since no UI yet.
                //self.reload_everything = true;
                self.dataset.reload_processes = true;
                
                if self.dataset.halt_execution { continue }
                
                for func in self.update_frame_symbols.iter() {
                    (*func)(&mut self.dataset)
                }
            }
        }
    }
    
    fn hotload_processes(&mut self) {    
        // All process references need to be cleared first, so the OS cache can be cleared as well.
        
        self.update_frame_symbols.clear();
        self.update_cycle_symbols.clear();
        self.update_process_binaries.clear();
        
        // All process types can then be loaded
        self.update_process_binaries = match Application::load_process_binaries(&self.process_binaries_path, &self.update_melody_path) {
            Some(value) => value,
            None        => { 
                println!("Failed to load process binaries. Execution Halted"); 
                self.dataset.halt_execution = true; 
                return
            }
        };
        
        // Symbols can then be loaded and stored        
        //self.update_frame_symbols = Application::load_functions(&self.update_process_binaries, "_per_frame");
        self.update_cycle_symbols = Application::load_functions(&self.update_process_binaries, "per_cycle");
        
        let mut frame_symbols: Vec<fn(&mut DB) -> ()> = Vec::new();

        for process in self.update_process_binaries.iter() {            
            let func: fn(&mut DB) -> () = unsafe {
                match process.symbol::<()>("per_frame") {
                    Err (why)       => { println! ("{}", why); continue },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            frame_symbols.push(func);
        }
        //println!("Loaded {} {} symbols", symbols.len(), name);
        self.update_frame_symbols = frame_symbols;
        
    }
    
    fn load_functions (processes: &Vec<DynamicLibrary>, name: &str) -> Vec<fn() -> ()> {        
        let mut symbols: Vec<fn() -> ()> = Vec::new();

        for process in processes.iter() {            
            let func: fn() -> () = unsafe {
                match process.symbol::<()>(name) {
                    Err (why)       => { println! ("{}", why); continue },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            symbols.push(func);
        }
        //println!("Loaded {} {} symbols", symbols.len(), name);
        symbols
    }
    
    
    fn load_process_binaries (binaries_path: &Path, melody_path: &Path) -> Option<Vec<DynamicLibrary>> {
        //println!("Loading process binaries: ");
        let mut processes: Vec<DynamicLibrary>    = Vec::new();
        
        let mut melody = BufferedReader::new(
        match File::open(melody_path) {
            Err(why)    => { println!("{}", why); return None }
            Ok(value) => { value }
        });
        
        for line in melody.lines() {
        
            let mut name : String = line.unwrap();
            name.pop();
            //name; // FIXME: Remove empty lines and newline char when necessary.
            let filename = "lib".to_string() + name.clone() + ".so".to_string();
            let binary_path = binaries_path.clone().join(filename.clone());
            if !binary_path.exists() { 
                println!("ERROR: '{}' doesn't exist. Did you misspell it?", filename); 
                return None
            }
            
            println!("Loading {}", binary_path.display());

            let binary = match DynamicLibrary::open(Some(&binary_path)) {
                Err (why)       => { println! ("ERROR: {}", why); return None; }
                Ok  (binary)    => { binary }
            };
            //println!("Loaded!");
            processes.push(binary);
            //println!("Found {} in {}", name, binary_path.display());
        }        
        Some(processes)
    }
}