//Scheduler will call execution schedules.

// Scheduler's role:
//


/*
    extern crate variable_update_schedule;
    extern crate fixed_update_schedule;

    // this one will be function pointered, and that's it!
    fn exec(fnpointer_get_data) {
        let data = fnpointer_get_data();

        variable_update_list.exec(data);

        if whatever {
            fixed_update_list.exec(data);
        }
    }
*/

// schedule locations will have to be loaded by scraping the ./../schedules/???/schedule/target dirs

extern crate common;
extern crate fixed_update_schedule;
extern crate variable_update_schedule;

use common::Data;

#[no_mangle]
pub fn exec(data: &mut Data) {
    //NOTE: I want to have the simulation update with a maximum speed
    //      so, the simulation can go as slow as it needs but it can't go faster than X
    let     max_fps                     : u64 = 60;
    let     target_frame_time           : u64 = 1000000000 / max_fps;
    let mut last_frame_time             : u64 = precise_time_ns();
    let mut last_cycle_time             : u64 = precise_time_ns();

    while !data.scheduler.quit {

        // variable update

        let current_time = precise_time_ns();
        data.scheduler.delta_time = current_time - last_cycle_time;

        variable_update.exec(&mut data);

        last_cycle_time = current_time;

        // fixed update

        if current_time - last_frame_time < target_frame_time {
            continue
        }

        fixed_update.exec(&mut data);

        last_frame_time = current_time;
    }
}
