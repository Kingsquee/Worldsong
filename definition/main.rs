extern crate time;

use time::precise_time_ns;

use std::mem;
use std::io::fs;
use std::io::File;
use std::io::BufferedReader;
use std::dynamic_lib::DynamicLibrary;

fn main() {
    let mut app = Application::new();
    app.exec();
}

struct Application {
    reload                      :   bool,
    paused                      :   bool,
    halted                      :   bool,

    update_exec_symbols         :   Vec<fn() -> ()>,

    update_routine_binaries     :   Vec<DynamicLibrary>,
    database_binaries           :   Vec<DynamicLibrary>,
    
    database_names              :   Vec<String>,
    
    update_melody_path          :   Path,
    database_binaries_path      :   Path,
    routine_binaries_path       :   Path,
}

impl Application {

    pub fn new() -> Application {
        let mut app = Application {
            reload                      :   false,
            paused                      :   false,
            halted                      :   false,
            
            update_exec_symbols         :   Vec::new(),
            
            update_routine_binaries     :   Vec::new(),
            database_binaries           :   Vec::new(),
            
            database_names              :   Vec::new(),
            
            update_melody_path          :   Path::new("./routines/update.melody"),
            database_binaries_path      :   Path::new("./databases/binaries/"),
            routine_binaries_path       :   Path::new("./routines/binaries"),
        };
        app.hotload_databases();
        app.hotload_routines();
        app
    }
    
    fn exec(&mut self) {       
        let     wait_time  : u64 = 1000000000 / 1; //60
        let mut next_tick  : u64 = precise_time_ns();
        
        loop {   
            let current_time_ns = precise_time_ns();
            
            if current_time_ns >= next_tick {         
            
                next_tick += wait_time;
                
                if self.reload == true {
                    self.hotload_routines();
                    self.reload = false;
                }
                
                //DEBUG: hotload every frame, since no UI yet.
                self.hotload_routines();
                
                if self.halted { continue }
                
                if self.paused { continue }
                for func in self.update_exec_symbols.iter() {
                    (*func)()
                }
            }
        }
    }
    
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
                let filename = x.filename_str();
                //println!("Found {}", filename)
                let extension = x.extension_str();
                
                if filename.is_none() || !filename.unwrap().starts_with("lib") || extension.is_none() || extension.unwrap() != "so" { continue; }
                
                let binary = match DynamicLibrary::open(Some(x)) {
                    Err (why)       => { fail! ("Error loading database binary: {}", why) }
                    Ok  (binary)    => { binary }
                };
                database_binaries.push(binary);
                
                let name = String::from_str(x.filename_str().unwrap());
                database_names.push(name);
                
            }
        }
        //println!("Loaded {} databases", database_binaries.len());
        
        self.database_binaries = database_binaries;
        self.database_names = database_names;
    }
    
    fn hotload_routines(&mut self) {
    
        self.halted = false;
    
        // All routine references need to be cleared first, so the OS cache can be cleared as well.
        
        self.update_exec_symbols.clear();

        self.update_routine_binaries.clear();
        
        // All routine types can then be loaded
        
        self.update_routine_binaries = match Application::load_routine_binaries(&self.routine_binaries_path, &self.update_melody_path) {
            Some(value) => value,
            None        => { 
                println!("Failed loading Update Processes. Execution Halted"); 
                self.halted = true; 
                return;
            }
        };
        
        // Symbols can then be loaded
        self.update_exec_symbols = Application::load_functions(&self.update_routine_binaries, "exec");
        
        // Finally, databases can be relinked.
        Application::link_databases(&self.update_routine_binaries, &self.database_binaries, &self.database_names);

    }

    /// @Assuptions: Databases and Database_names have the same length
    fn link_databases (routines: &Vec<DynamicLibrary>, databases: &Vec<DynamicLibrary>, database_names: &Vec<String>) {
        //println!("Linking databases to routines");
        assert!(databases.len() == database_names.len());
    
        for routine in routines.iter() {
            // Load database functions. If they fail, continue to the next routine.
            
            //println!("Loading dbrequest");
            let dbrequest_func: fn<'a> () -> Vec<&'a str> = unsafe {
                match routine.symbol::<()>("_dbrequest") {
                    Err (why)       => { println! ("{}", why); continue; },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            
            //println!("Loading dbassign");
            let dbassign_func: fn(dbs: &Vec<&DynamicLibrary>) -> () = unsafe {
                match routine.symbol::<()>("_dbassign") {
                    Err (why)       => { println! ("{}", why); continue; },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            
            //println!("Loaded DBs: {}", database_names);
            
            let requested_database_names: Vec<&str> = dbrequest_func();
            //println!("Requested DBs: {}", requested_database_names);
            let mut requested_database_binaries: Vec<&DynamicLibrary> = Vec::new();
            
            for requested_database_name in requested_database_names.iter() {
                let mut i: uint = 0;
                while i < database_names.len() {
                    if *requested_database_name == database_names[i].as_slice() {
                        requested_database_binaries.push(&databases[i]);
                        //println!("Linked {}", database_names[i]);
                        i += 1;
                        continue
                    }
                    //println!("Could not find {}, skipping.", database_names[i]); //TODO: Should halt application instead? Return None?
                    i += 1;
                }
            }
            //println!("Sending {} binares to dbassign", requested_database_binaries.len());
            dbassign_func(&requested_database_binaries);
        }
        
        // TODO: call dbrequest to see what dbs the routines want
        //          return array of references to each requested db dynamiclibrary instance, so the routine may call the get_ref() function.
        
    }
    
    fn load_functions (routines: &Vec<DynamicLibrary>, name: &str) -> Vec<fn() -> ()> {        
        let mut symbols: Vec<fn() -> ()> = Vec::new();

        for routine in routines.iter() {            
            let func: fn() -> () = unsafe {
                match routine.symbol::<()>(name) {
                    Err (why)       => { println! ("{}", why); continue; },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            symbols.push(func);
        }
        //println!("Loaded {} {} symbols", symbols.len(), name);
        symbols
    }
    
    
    fn load_routine_binaries (binaries_path: &Path, melody_path: &Path) -> Option<Vec<DynamicLibrary>> {
        //println!("Loading routine binaries: ");
        let mut routines: Vec<DynamicLibrary>    = Vec::new();
        
        let mut melody = BufferedReader::new(
        match File::open(melody_path) {
            Err(why)    => { println!("{}", why); return None; }
            Ok(value) => { value }
        });
        
        for line in melody.lines() {
        
            let mut name : String = line.unwrap();
            name.pop_char();
            //name; // FIXME: Remove empty lines and newline char when necessary.
            
            let binary_path = binaries_path.clone().join(name.clone().append(".routine"));
            if !binary_path.exists() { 
                println!("WARNING: '{}' doesn't exist. Did you misspell it?", name); 
                continue; 
            }

            let binary = match DynamicLibrary::open(Some(&binary_path)) {
                Err (why)       => { println! ("ERROR: {}", why); return None; }
                Ok  (binary)    => { binary }
            };
            
            routines.push(binary);
            //println!("Found {} in {}", name, binary_path.display());
        }        
        Some(routines)
    }
}