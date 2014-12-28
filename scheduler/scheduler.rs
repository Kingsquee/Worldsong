extern crate common;
extern crate fixed_update;
extern crate variable_update;

extern crate time;

use common::state::Data;
use time::precise_time_ns;

#[no_mangle]
pub fn run(data: &mut Data) {
    //NOTE: I want to have the simulation update with a maximum speed
    //      so, the simulation can go as slow as it needs but it can't go faster than X
    let     max_fps                     : u64 = 60;
    let     target_frame_time           : u64 = 1000000000 / max_fps;
    let mut last_frame_time             : u64 = precise_time_ns();
    let mut last_cycle_time             : u64 = precise_time_ns();

    while !data.core.quit && !data.core.reload && !data.core.reset {

        // variable update
        let current_time = precise_time_ns();
        data.core.delta_time = current_time - last_cycle_time;

        variable_update::execute(data);

        last_cycle_time = current_time;

        // fixed update
        if current_time - last_frame_time < target_frame_time {
            continue
        }

        fixed_update::execute(data);

        last_frame_time = current_time;
    }
}
