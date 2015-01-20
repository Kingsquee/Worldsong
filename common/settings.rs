// How should each library crate be compiled? Statically or dynamically?

pub fn get_process_lib_type() -> &'static str {
    "dylib"
}

pub fn get_schedules_lib_type() -> &'static str {
    "dylib"
}

pub fn get_scheduler_lib_type() -> &'static str {
    "dylib"
}

pub fn get_struct_lib_type() -> &'static str {
    "dylib"
}

pub fn get_state_lib_type() -> &'static str {
    "dylib"
}
