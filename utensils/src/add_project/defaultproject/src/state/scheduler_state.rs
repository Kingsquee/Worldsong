use time::precise_time_ns;

data! (
    SchedulerState {
        current_time        : u64 = precise_time_ns()
        max_fps             : u64 = 60
        target_frame_time   : u64 = 1000000000 / max_fps
        last_frame_time     : u64 = precise_time_ns()
        last_cycle_time     : u64 = precise_time_ns()
    }
);