#![crate_name="PlaceholderProcess"]
extern crate dataset;

use dataset::DB;

/// Called asap. TODO: Change to deltatime
#[no_mangle]
pub fn per_cycle() {
    
}

/// Called every 1/60th of a second.
#[no_mangle]
pub fn per_frame(data: &mut DB) {
    println!("{}", data.color_r);
    (*data).color_r += 1;
}