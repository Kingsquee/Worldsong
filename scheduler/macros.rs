// Need a macro for execution sequence to keep typing down

/*
execution_sequence! {
    process_name_one(data.whatever)
    process_name_two(data.whateverelse)
}
    into

    //#[path="processes/process_name_one/process_name_one.rs"]
    extern crate process_name_one;
    extern crate process_name_two;

    fn exec(Data data) {
        process_name_one::exec(data.whatever);
        process_name_two::exec(data.whateverelse);
    }
*/

#![macro_esacpe]

#[macro_export]
($($process_name:ident($($param_name:ident)*))+) => {
    $(
        extern crate $process_name;
    )+

    fn exec(data: &mut Data) {
        $(
            $process_name::exec($($param_name,)+)
        )+
    }
}
