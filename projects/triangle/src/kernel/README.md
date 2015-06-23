The kernel is the main point of entry for the application, and, assuming the scheduler was compiled as a dylib, performs the hotloading.

If the scheduler was compiled to a dylib, 'dynamic_kernel.rs' will be built, otherwise 'static_kernel.rs'.