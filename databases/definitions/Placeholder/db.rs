#![crate_name = "PlaceholderData"]

pub struct PlaceholderDB {
    pub a: u16,
    pub b: u8
}

static mut db: PlaceholderDB = PlaceholderDB { a: 12, b: 3 };

#[no_mangle]
/// Saves the database struct instance to the drive
pub fn save(filepath: String) {

}

#[no_mangle]
/// Loads the database struct instance from the drive
pub fn load(filepath: String) {

}

#[no_mangle] 
/// Returns a reference to the database struct instance
pub fn get_ref() -> PlaceholderDB {
    println!("Inside database's get_ref. Returning database reference!");
    unsafe { db }
}