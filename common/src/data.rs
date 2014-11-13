use sdl2;
use sdl2::render::Renderer;

// The data! macro combines 'struct' and 'impl' syntax for the lazy typists
// in all of us. It's definition is in common/macros.rs

// Note that by using this macro, struct fields with dependencies limit
// your struct packing options due to order of initialiation.
// See: window.renderer

// This can be avoided by initializing values by loading config files,
// rather than appropriating hard coded values such as here.
data!( 
    kernel: KernelState {
        delta_time:             u64             = 0
        load_processes:         bool            = true
        quit:                   bool            = false
    }
    window: WinState {
        title:                  String          = "Worldsong".to_string()
        width:                  int             = 640
        height:                 int             = 480
        renderer:               Renderer        = init_renderer(title.as_slice(), width, height)
        first_focus:            bool            = true
        opengl_major_version:   u32             = 3
        opengl_minor_version:   u32             = 0
    }

    sim: SimState {
        pause:                  bool            = false
        color_r:                u8              = 0
        color_g:                u8              = 0
        color_b:                u8              = 255
    }
)

pub fn init_renderer(title: &str, width: int, height: int) -> Renderer {
    let window = sdl2::video::Window::new(title, sdl2::video::PosCentered, sdl2::video::PosCentered, width, height, sdl2::video::OPENGL).unwrap();
    let renderer = sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED).unwrap();
    renderer
}
