static mut i: int = 0;

// I want this damnit
/*
databases!(
    ("a", var_a),
    ("b", var_b),
    //etc
)
*/



pub extern fn _dbassign <T> (app: &Application) {
    let a = app.get_db("a");
    let b = app.get_db("b");
    let c = app.get_db("c");
}


#[no_mangle]
/// Execute your logic here. It's called at 60hz unless the game is lagging.
pub extern fn exec () {
    unsafe {
        println!("Exececuted {} times", i);
        i += 1;
    }
}

/*
// relationships
struct Entities {
    instance_relationship:  i32,
    type_relationship:      i32
}*/