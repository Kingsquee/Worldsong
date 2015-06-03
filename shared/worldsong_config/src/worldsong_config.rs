pub fn get_process_lib_type() -> &'static str {
    "dylib"
}

pub fn get_schedule_lib_type() -> &'static str {
    "dylib"
}

pub fn get_scheduler_lib_type() -> &'static str {
    "dylib"
}

pub fn get_state_lib_type() -> &'static str {
    "dylib"
}

pub fn get_default_rustc_flags() -> Vec<&'static str> {
    vec!["-C", "opt-level=3", "-C", "debuginfo=2"]
}