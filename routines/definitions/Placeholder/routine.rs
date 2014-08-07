extern crate PlaceholderData;

use std::dynamic_lib::DynamicLibrary;
use std::mem;

static mut i: int = 1;
/*
databases!(
    ("a", var_a),
    ("b", var_b),
    //etc
) //turns into: 
*/

/// hidden macro function
/// returns the database names we want as a string vec
#[no_mangle]
pub fn _dbrequest<'a> () -> Vec<&'a str> {
    vec!["libPlaceholderData.so"]
}

/// hidden macro function
/// carries the databases we want, in the same order as defined in _dbrequest
#[no_mangle]
pub fn _dbassign (dbs: &Vec<&DynamicLibrary>) {
    println!("Assigning database to routine.")
    
    // find the get_ref symbol in each dynamiclibrary
    // call each symbol and assign the output to var_a and var_b, etc
    
    // HOLY CRAP ALMOST THEERE
    let mut var_db : PlaceholderData::PlaceholderDB;
    for db in dbs.iter() {
        let get_ref: fn() -> PlaceholderData::PlaceholderDB = unsafe {
            match db.symbol::<()>("get_ref") {
                Err (why)       => { println! ("{}", why); continue; },
                Ok  (func)      => { mem::transmute(func) }
            }
        };
        
        var_db = get_ref();
        println!("Database assigned!");
        
        println!("Incrementing variable 'a' inside database");
        println!("A is {}", var_db.a);
        var_db.a += 1;
        println!("A is now {}", var_db.a);
        var_db.a += 1;
        println!("A is now {}!", var_db.a);
        
    }
    
}

#[no_mangle]
/// Execute your logic here. It's called at 60hz unless the game is lagging.
pub fn exec () {
    unsafe {
        println!("Modification test");
        println!("Exececuted {} times since last reload", i);
        i -= 1;
    }
}