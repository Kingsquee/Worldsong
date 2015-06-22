extern crate time;
extern crate state;
extern crate fixed_update;
extern crate variable_update;

use state::Data;
use time::precise_time_ns;

#[no_mangle]
pub fn run(data: &mut Data) {
    while !data.core_state.quit && !data.core_state.reload && !data.core_state.reset {

        // variable update
        data.scheduler_state.current_time = precise_time_ns();
        data.core_state.delta_time = data.scheduler_state.current_time - data.scheduler_state.last_cycle_time;

        variable_update::execute(data);

        data.scheduler_state.last_cycle_time = data.scheduler_state.current_time;

        // fixed update
        if data.scheduler_state.current_time - data.scheduler_state.last_frame_time < data.scheduler_state.target_frame_time {
            continue
        }

        fixed_update::execute(data);

        data.scheduler_state.last_frame_time = data.scheduler_state.current_time;
    }
}
