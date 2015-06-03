extern crate worldsong_hierarchy;
extern crate state;
extern crate scheduler;

use self::state::Data;

pub fn exec() {
    let mut data = Data::new();

    loop {
        // Passing the hotloaded constructor to the hotloaded scheduler execution function.
        println!("Calling run");
        scheduler::run(&mut data);

        if data.core_state.quit {
            println!("Quitting.");
            break
        }
        else if data.core_state.reload {
            println!("Cannot reload scheduler: the kernel was compiled statically.");
            data.core_state.reload = false;
        }
        // TODO: Would be nice to have this load the latest state::Data from disk.
        else if data.core_state.reset {
            println!("Resetting state...");
            data = Data::new();
            data.core_state.reset = false;
        }
    }
}