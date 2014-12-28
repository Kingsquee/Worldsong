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

pub fn get_common_lib_type() -> &'static str {
    "dylib"
}
