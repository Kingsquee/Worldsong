extern crate sdl2;

use self::sdl2::render::Renderer;

data! {
    GraphicsState {
        title:                  String          = "Worldsong".to_string()
        width:                  isize           = 640
        height:                 isize           = 480
        renderer:               Renderer        = init_renderer(title.as_slice(), width, height)
        first_focus:            bool            = true
        opengl_major_version:   u32             = 3
        opengl_minor_version:   u32             = 0
    }
}

pub fn init_renderer(title: &str, width: int, height: int) -> Renderer {
    let window = sdl2::video::Window::new(title, sdl2::video::WindowPos::PosCentered, sdl2::video::WindowPos::PosCentered, width, height, sdl2::video::OPENGL).unwrap();
    let renderer = sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED).unwrap();
    renderer
}
