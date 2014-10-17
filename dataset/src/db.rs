#![feature(phase)]
#[phase(plugin, link)] 
extern crate utils;
extern crate debug;
extern crate sdl2;

use sdl2::video::Window;
use sdl2::render::Renderer;

db!(
    reload_processes:       bool                        = false
    halt_execution:         bool                        = false
    stop_execution:         bool                        = false
    color_r:                u8                          = 0
    color_g:                u8                          = 0
    color_b:                u8                          = 255
    window_width:           int                         = 640
    window_height:          int                         = 480
    window_title:           &'static str                = "Worldsong"
    opengl_major_version:   u32                         = 3
    opengl_minor_version:   u32                         = 0
    renderer:               Option<Renderer<Window>>    = None
)

pub fn initialize(data: &mut DB) {
    let window = match sdl2::video::Window::new(data.window_title, sdl2::video::PosCentered, sdl2::video::PosCentered, data.window_width, data.window_height, sdl2::video::OPENGL) 
    {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };

    data.renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) 
    {
        Ok(renderer) => Some(renderer),
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
}