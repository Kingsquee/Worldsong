extern crate time;

use time::precise_time_ns;

use std::mem;
use std::io::fs;
use std::io::File;
use std::io::BufferedReader;
use std::dynamic_lib::DynamicLibrary;

fn main() {
    let mut app = Application::new();
    Application::exec(&mut app);
}

// TODO: Pack this better
struct Application {
    reload                      :   bool,
    paused                      :   bool,
    halted                      :   bool,
    
    database_binaries           :   Vec<DynamicLibrary>,
    database_names              :   Vec<String>,
    
    pause_routine_binaries      :   Vec<DynamicLibrary>,
    pause_exec_symbols          :   Vec<extern fn() -> ()>,
    pause_melody_path           :   Path,
    
    unpause_routine_binaries    :   Vec<DynamicLibrary>,
    unpause_exec_symbols        :   Vec<extern fn() -> ()>,
    unpause_melody_path         :   Path,
    
    always_routine_binaries     :   Vec<DynamicLibrary>,
    always_exec_symbols         :   Vec<extern fn() -> ()>,
    always_melody_path          :   Path,
    
    database_binaries_path      :   Path,
    routine_binaries_path       :   Path,
}

impl Application {

    pub fn new() -> Application {
        let mut app = Application {
            reload                      :   false,
            paused                      :   false,
            halted                      :   false,
            
            database_binaries           :   Vec::new(),
            database_names              :   Vec::new(),
            
            always_routine_binaries     :   Vec::new(),
            always_exec_symbols         :   Vec::new(),
            always_melody_path          :   Path::new("./routines/always.melody"),
            
            pause_routine_binaries      :   Vec::new(),
            pause_exec_symbols          :   Vec::new(),
            pause_melody_path           :   Path::new("./routines/pause.melody"),
            
            unpause_routine_binaries    :   Vec::new(),
            unpause_exec_symbols        :   Vec::new(),
            unpause_melody_path         :   Path::new("./routines/unpause.melody"),
            
            database_binaries_path      :   Path::new("./databases/binaries/"),
            routine_binaries_path       :   Path::new("./routines/binaries"),
        };
        app.hotload_databases();
        app.hotload_routines();
        app
    }
    

    fn exec(app: &mut Application) {       
        let     wait_time  : u64 = 1000000000 / 1; //60
        let mut next_tick  : u64 = precise_time_ns();
        
        loop {   
            let current_time_ns = precise_time_ns();
            
            if current_time_ns >= next_tick {         
            
                next_tick += wait_time;
                
                if app.reload == true {
                    app.hotload_routines();
                    app.reload = false;
                }
                
                //DEBUG: hotload every frame, since no UI yet.
                app.hotload_routines();
                
                if app.halted { continue; }
                
                for func in app.always_exec_symbols.iter() {
                    (*func)()
                }
                
                if app.paused { 
                    for func in app.pause_exec_symbols.iter() {
                        (*func)()
                    }
                } else {
                    for func in app.unpause_exec_symbols.iter() {
                        (*func)()
                    }
                }
            }
        }
    }
    
    //TODO: Test more
    fn hotload_databases (&mut self) {
        println!("Loading databases");
        
        let mut database_binaries:  Vec<DynamicLibrary> = Vec::new();
        let mut database_names:     Vec<String> = Vec::new();
        let dir = match fs::readdir(&self.database_binaries_path) {
            Err (why)   => { fail!("Could not load database binaries directory. {}", why) }
            Ok  (dir)   => dir
        };
        
        for x in dir.iter() {
            if x.is_file() {
            
                let extension = x.extension_str();
                
                if extension.is_none() { continue; } 
                else if extension.unwrap() != ".data" { continue; }
                
                let binary = match DynamicLibrary::open(Some(x)) {
                    Err (why)       => { fail! ("Error loading database binary: {}", why) }
                    Ok  (binary)    => { binary }
                };
                database_binaries.push(binary);
                
                let name = String::from_str(x.filename_str().unwrap());
                database_names.push(name);
                
            }
        }
        println!("Loaded {} databases", database_binaries.len());
        
        self.database_binaries = database_binaries;
        self.database_names = database_names;
    }
    
    fn hotload_routines(&mut self) {
    
        // All routine references need to be cleared first, so the OS cache can be cleared as well.
        
        self.always_exec_symbols.clear();
        self.pause_exec_symbols.clear();
        self.unpause_exec_symbols.clear();
        
        self.always_routine_binaries.clear();
        self.pause_routine_binaries.clear();
        self.unpause_routine_binaries.clear();
        
        // All routine types can then be loaded
        
        self.always_routine_binaries = match Application::load_routine_binaries(&self.routine_binaries_path, &self.always_melody_path) {
            Some(value) => value,
            None        => { 
                println!("Failed loading Always Processes. Execution Halted"); 
                self.halted = true; 
                return;
            }
        };

        self.pause_routine_binaries = match Application::load_routine_binaries(&self.routine_binaries_path, &self.pause_melody_path) {
            Some(value) => value,
            None        => { 
                println!("Failed loading Pause Processes. Execution Halted"); 
                self.halted = true; 
                return;
            }
        };
        
        self.unpause_routine_binaries = match Application::load_routine_binaries(&self.routine_binaries_path, &self.unpause_melody_path) {
            Some(value) => value,
            None        => { 
                println!("Failed loading Unpause Processes. Execution Halted"); 
                self.halted = true; 
                return;
            }
        };
        
        // Symbols can then be loaded
        
        self.always_exec_symbols =  Application::load_functions(&self.always_routine_binaries,  "exec");
        self.pause_exec_symbols =   Application::load_functions(&self.pause_routine_binaries,   "exec");
        self.unpause_exec_symbols = Application::load_functions(&self.unpause_routine_binaries, "exec");
        
        /*
        let dbrequest =    Application::load_functions(routines, "_dbrequest");
        let dbassigns =    Application::load_functions(routines, "_dbassign");
        */
        
        // Finally, databases can be relinked.
        
        // TODO: call dbrequest to see what dbs the routines want
        //          return array of references to each requested db struct instance 

    }
    
    fn load_functions (routines: &Vec<DynamicLibrary>, name: &str) -> Vec<extern fn() -> ()> {
        println!("Loading {} symbols", name);
        
        let mut exec_symbols: Vec<extern fn() -> ()> = Vec::new();

        for binary in routines.iter() {            
            let func: extern fn() -> () = unsafe {
                match binary.symbol::<()>(name) {
                    Err (why)       => { println! ("{}", why); continue; },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            exec_symbols.push(func);
        }
        println!("Loaded {} {} funcs", exec_symbols.len(), name);
        exec_symbols
    }
    
    
    fn load_routine_binaries (binaries_path: &Path, melody_path: &Path) -> Option<Vec<DynamicLibrary>> {
        println!("Loading routine binaries: ");
        let mut routines: Vec<DynamicLibrary>    = Vec::new();
        
        let mut melody = BufferedReader::new(
        match File::open(melody_path) {
            Err(why)    => { println!("{}", why); return None; }
            Ok(value) => { value }
        });
        
        for line in melody.lines() {
        
            let name : String = line.unwrap().append(".routine");
            //name.pop_char(); // FIXME: Remove empty lines and newline char when necessary.
            
            let binary_path = binaries_path.clone().join(name.clone());
            if !binary_path.exists() { 
                println!("WARNING: '{}' doesn't exist. Did you misspell it?", name); 
                continue; 
            }

            let binary = match DynamicLibrary::open(Some(&binary_path)) {
                Err (why)       => { println! ("ERROR: {}", why); return None; }
                Ok  (binary)    => { binary }
            };
            
            routines.push(binary);
            println!("Found {} in {}", name, binary_path.display());
        }        
        Some(routines)
    }
}