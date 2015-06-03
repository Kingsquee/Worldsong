#![feature(std_misc)]

// When compiling everything as rlibs, switch from a dynamic kernel to a static kernel.
// This allows compiling a worldsong project into a single binary.
mod dynamic_kernel;
//mod static_kernel;

fn main() {
    dynamic_kernel::exec();
    //static_kernel::exec();
}