#![feature(phase)]
#[phase(plugin)] 
extern crate Utils;

databases!(
    placeholder_db  { placeholder = PlaceholderData }
)

#[no_mangle]
pub fn exec () {
        if placeholder_db().a == 0 { placeholder_db().a = 2; }
        placeholder_db().a *= 2;
        println!("placeholder db's a is {}", placeholder_db().a);
}