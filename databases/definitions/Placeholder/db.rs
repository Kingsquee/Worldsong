#![crate_name = "PlaceholderData"]

pub struct DB {
    pub a: u16,
    pub b: u8
}

impl DB {
    pub fn new() -> DB {
        let pdb = DB {
            a: 0,
            b: 0,
        };
        pdb
    }
}

static mut db: DB = DB { a: 0, b: 0 };

#[no_mangle]
/// Saves the database struct instance to the drive
pub fn save(filepath: String) {

}

#[no_mangle]
/// Loads the database struct instance from the drive
pub fn load(filepath: String) {

}

#[no_mangle] 
/// Returns a reference to the database struct instance.
pub fn get_ref<'a>() -> &'a mut DB {
    //println!("Inside database's get_ref. Returning database reference!");
    unsafe { &mut db }
}