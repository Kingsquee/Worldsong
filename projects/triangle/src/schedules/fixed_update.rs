#[macro_use] extern crate schedule_macro;

schedule!(
    graphics_process(graphics_state)
    input_process(graphics_state, core_state)
);
