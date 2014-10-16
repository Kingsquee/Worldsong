#![feature(phase)]
#[phase(plugin, link)] 
extern crate utils;
extern crate glutin;

use glutin::{ Window, WindowBuilder};

db!(
    reload_processes:       bool            = false
    halt_execution:         bool            = false
    stop_execution:         bool            = false
    color_r:                u8              = 0
    color_g:                u8              = 0
    color_b:                u8              = 255
    window_width:           uint            = 640
    window_height:          uint            = 480
    window_title:           &'static str    = "Worldsong"
    opengl_major_version:   u32             = 3
    opengl_minor_version:   u32             = 0
    window:                 Window          = Window::new().unwrap()
)

pub fn initialize(data: &DB) {
    //data.window.set_inner_size(data.window_width, data.window_height);
    data.window.set_title(data.window_title);
    //unsafe { data.window.make_current() }
}