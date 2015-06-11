#[macro_use] extern crate schedule_macro;

schedule!(
    graphics_process(simulation_state, graphics_state)
    some_process(simulation_state)
    another_process()
);
