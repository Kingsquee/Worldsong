extern crate time;

use time::precise_time_ns;

use std::mem;
use std::io::fs;
use std::io::File;
use std::io::BufferedReader;
use std::dynamic_lib::DynamicLibrary;

static PAUSABLE_PROCESSES_PATH          : &'static str = "./processes/pausable/binaries";
static PAUSABLE_PROCESSES_MELODY_PATH   : &'static str = "./processes/pausable/melody";
static UNPAUSABLE_PROCESSES_PATH        : &'static str = "./processes/unpausable/binaries";
static UNPAUSABLE_PROCESSES_MELODY_PATH : &'static str = "./processes/unpausable/melody";
static DATABASES_PATH                   : &'static str = "./databases/binaries/";

fn main() {
    let mut app = Application::new();
    Application::exec(&mut app);
    
}

struct Application {
    reload                      :   bool,
    paused                      :   bool,
    halted                      :   bool,
    database_binaries           :   Vec<DynamicLibrary>,
    database_names              :   Vec<String>,
    pausable_processes          :   Vec<DynamicLibrary>,
    unpausable_processes        :   Vec<DynamicLibrary>,
    pausable_exec_functions     :   Vec<extern fn() -> ()>,
    unpausable_exec_functions   :   Vec<extern fn() -> ()>,
}

impl Application {

    pub fn new() -> Application {
        let mut app = Application {
            reload                      :   false,
            paused                      :   false,
            halted                      :   false,
            database_binaries           :   Vec::new(),
            database_names              :   Vec::new(),
            pausable_processes          :   Vec::new(),
            unpausable_processes        :   Vec::new(),
            pausable_exec_functions     :   Vec::new(),
            unpausable_exec_functions   :   Vec::new(),
        };
        Application::load_databases (&mut app);
        Application::load_processes (&mut app);
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
                    if Application::load_processes(app).is_none() { println!("Execution Halted"); app.halted = true; };
                    app.reload = false;
                }
                
                if app.halted { continue; }
                for func in app.unpausable_exec_functions.iter() {
                    (*func)()
                }
                
                if app.paused { continue; }
                for func in app.pausable_exec_functions.iter() {
                    (*func)()
                }
                
            }
        }
    }
    
    fn load_databases (app: &mut Application) {
        println!("Loading databases");
        
        let mut database_binaries:  Vec<DynamicLibrary> = Vec::new();
        let mut database_names:     Vec<String> = Vec::new();
        let dir = match fs::readdir(&Path::new(DATABASES_PATH)) {
            Err (why)   => { fail!("Could not load database binaries directory. {}", why) }
            Ok  (dir)   => dir
        };
        
        for x in dir.iter() {
            if x.is_file() {
                let binary = match DynamicLibrary::open(Some(x)) {
                Err (why)       => { fail! ("Error loading database binary: {}", why) }
                Ok  (binary)    => { binary }
                };
                database_binaries.push(binary);
                
                let name = x.filename_str();
                database_names.push(name.to_str());
            }
        }
        println!("Loaded {} databases", database_binaries.len());
        
        app.database_binaries = database_binaries;
        app.database_names = database_names;
        
    }
    
/*
    pub fn get_db<T> (name: String) -> T {
        
    }
    */
    fn load_processes (app: &mut Application) -> Option<()> {
    
        println!("Loading pausable process binaries");
        app.pausable_processes.clear();
        app.pausable_processes = match Application::load_process_binaries(&Path::new(PAUSABLE_PROCESSES_PATH), &Path::new(PAUSABLE_PROCESSES_MELODY_PATH)) {
            Some(value) => value,
            None        => { return None }
        };
        
        println!("Loading unpausable process binaries");
        app.unpausable_processes.clear();
        app.unpausable_processes = match Application::load_process_binaries(&Path::new(UNPAUSABLE_PROCESSES_PATH), &Path::new(UNPAUSABLE_PROCESSES_MELODY_PATH)) {
            Some(value) => value,
            None        => { return None; }
        };
        /*
        println!("Loading dbassigns symbols for pausable processes");
        let pausable_dbassigns = match Application::load_functions(&app.pausable_processes, "_dbassign") {
            Some(value) => value,
            None        => { return None; }
        }
        
        */
        println!("Loading exec symbols for pausable processes");
        app.pausable_exec_functions.clear();
        app.pausable_exec_functions = match Application::load_functions(&app.pausable_processes, "exec") {
            Some(value) => value,
            None        => { return None; }
        };
        
        println!("Loading exec symbols for unpausable processes");
        app.unpausable_exec_functions.clear();
        app.unpausable_exec_functions = match Application::load_functions(&app.unpausable_processes, "exec") {
            Some(value) => value,
            None        => { return None; }
        };
        
        Some(())
    }
    
    fn load_functions (processes: &Vec<DynamicLibrary>, name: &str) -> Option<Vec<extern fn() -> ()>> {
        let mut exec_functions: Vec<extern fn() -> ()> = Vec::new();
        
        for binary in processes.iter() {            
            let func: extern fn() -> () = unsafe {
                match binary.symbol::<()>(name) {
                    Err (why)       => { println! ("{}", why); continue; },
                    Ok  (func)      => { mem::transmute(func) }
                }
            };
            exec_functions.push(func);
        }
        println!("Loaded {} {} funcs", exec_functions.len(), name);
        Some(exec_functions)
    }
        
    fn load_process_binaries (binaries_path: &Path, melody_path: &Path) -> Option<Vec<DynamicLibrary>> {

        let mut processes: Vec<DynamicLibrary>    = Vec::new();
        
        let mut melody = BufferedReader::new(
        match File::open(melody_path) {
            Err(why)    => { println!("{}", why); return None; }
            Ok(value) => { value }
        });
        
        for line in melody.lines() {
        
            let mut name : String = line.unwrap();
            name.pop_char(); // Remove newline char.
            
            let binary_path = binaries_path.clone().join(name.clone());
            
            if !binary_path.exists() { continue; }

            let binary = match DynamicLibrary::open(Some(&binary_path)) {
                Err (why)       => { println! ("        Error loading process binary: {}", why); return None; }
                Ok  (binary)    => { binary }
            };
            
            processes.push(binary);
            println!("Found {} in {}", name, binary_path.display());
        }        
        Some(processes)
    }
}