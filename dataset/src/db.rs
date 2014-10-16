#![feature(phase)]
#[phase(plugin, link)] 
extern crate utils;

db!(
    reload_processes:       bool            = false
    halt:                   bool            = false
    stop:                   bool            = false
    color_r:                u8              = 12
    color_g:                u8              = 0
    color_b:                u8              = 255
    window_width:           u32             = 640
    window_height:          u32             = 480
    window_title:           &'static str    = "Worldsong"
    opengl_major_version:   u32             = 3
    opengl_minor_version:   u32             = 0
)