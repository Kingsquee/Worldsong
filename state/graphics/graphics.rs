// Symphony will generate:
// - a folder, state/$name
// - a file, state/$name/$name.rs, containing "pub struct $NameState { }" or equiv macro (data!)
//      (TODO: expand data! to support single layer struct or the current two-layer struct)
// - a Cargo.toml file with $name in appropriate places

// on compilation, an rlib will be generated for the datum
// followed by a child job compiling state.rs as a dylib
// followed by child jobs recompiling everything else to relink state.rs
extern crate lazystruct;
extern crate sdl2;

use sdl2;
use sdl2::render::Renderer;

lazystruct! {
    graphicsstate: GraphicsState {
        title:                  String          = "Worldsong".to_string()
        width:                  int             = 640
        height:                 int             = 480
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
