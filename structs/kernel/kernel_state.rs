#[macro_use]
extern crate common;

data! (
    KernelState {
        delta_time: u64     = 0
        reload:     bool    = false
        reset:      bool    = false
        quit:       bool    = false
    }
);
