#![feature(macro_rules)]

pub mod io {
    mod binary {
    // for databases, if serialize over the network, have a db_last_tick to compare, and send the delta.

        /*
        /// Saves the database struct instance to the drive
        pub fn save<T>(data: T, dir: &Path) {
            let filepath = dir.join("PlaceholderData");
            let display = filepath.display();
            // Open a file in write-only mode, returns 'IoResult<File>'
            let mut file = match File::create(&filepath) {
                Ok(file) => { file }
                Err(why) => { println!("couldn't create {}: {}", display, why.desc) return None }
            };
        }

        /// Loads the database struct instance from the drive
        pub fn load(data: T, dir: &Path) {
            let filepath = dir.join("PlaceholderData");
            let display = filepath.display();
            // Open the path in read-only mode, returns 'IoResult<File>'
            let mut file = match File::open(&filepath) {
                // The 'desc' field of 'IoError' is a string that describes the error
                Ok(file) => { file }
                Err(why) => { println!("couldn't open {}: {}", display, why.desc); return None }
            };
        }
        */
    }
}

#[macro_export]
macro_rules! db {
    ($($var:ident: $vartype:ty = $val:expr)+) => {
        #[no_mangle]
        #[allow(dead_code)]
        pub struct DB {
            $(
                pub $var: $vartype,
            )+
        }
        
        #[no_mangle] 
        #[allow(dead_code)]
        /// Creates a new database struct instance.
        impl DB {
            pub fn new() -> DB {
             DB {
                $(
                    $var: $val,
                )+
             }
            }
        }
    }
}