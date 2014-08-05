static mut i: int = 1;
/*
databases!(
    ("a", var_a),
    ("b", var_b),
    //etc
) //turns into: 

/// hidden macro function
/// returns the database names we want as csv
#[no_mangle]
pub fn _dbrequest<'a> () -> &'a str {
    "a,b"
}

/// hidden macro function
/// returns the databases we want, in the same order as defined in _dbrequest
#[no_mangle]
pub fn _dbassign <T> (dbs: Vec<&T>) {
    var_a = dbs.get(0);
    var_b = dbs.get(1);
}
*/
// Then we 

#[no_mangle]
/// Execute your logic here. It's called at 60hz unless the game is lagging.
pub fn exec () {
    unsafe {
        println!("Modification test");
        println!("Exececuted {} times since last reload", i);
        i -= 1;
    }
}