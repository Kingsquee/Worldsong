#![crate_type = "dylib"]
#![crate_name = "Utils"]
#![feature(macro_rules)]

#[macro_export]
macro_rules! databases {
    ($($fnname:ident { $varname:ident = $dbname:ident })+) => {
        mod _hack {
            $(
                extern crate $dbname;
            )+
            pub use std::dynamic_lib::DynamicLibrary;
            pub use std::mem;
        }
        $(
            static mut $varname : *mut _hack::$dbname::DB = 0 as *mut _hack::$dbname::DB;
        )+
        
        $(
            fn $fnname() -> &'static mut _hack::$dbname::DB {
                unsafe { &mut*$varname }
            }
        )+
    
        #[no_mangle]
        pub fn _dbrequest<'a> () -> Vec<&'a str> {
            vec![
            $(
                concat!("lib", stringify!($dbname),".so"),
            )+
            ]
        }
        
        /// hidden macro function
        /// carries the databases we want, in the same order as defined in _dbrequest
        #[no_mangle]
        #[allow(dead_assignment)]
        pub fn _dbassign (dbs: &Vec<&_hack::DynamicLibrary>) { 

            //println!("dbassign has recieved {} databases", dbs.len());
            
            let mut i: uint = 0;
            
            $(
                //println!("Assigning {} to routine.", stringify!($dbname))
                let get_ref: fn <'a> () -> &'a mut _hack::$dbname::DB = unsafe {
                    match dbs.get(i).symbol::<()>("get_ref") {
                        Err (why)       => { println! ("{}", why); return; }, // What to do for error handling?
                        Ok  (func)      => { _hack::mem::transmute(func) }
                    }
                };
                
                unsafe {
                    $varname = get_ref() as *mut _hack::$dbname::DB;
                }
                
                i += 1;
                //println!("{} assigned!", stringify!($dbname));
            )+
        
        }
    }
}
