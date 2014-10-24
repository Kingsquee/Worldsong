use sdl2;
use sdl2::video::Window;
use sdl2::render::Renderer;

data!( 
    kernel: KernelState {
        load_processes:         bool            = true
        quit:                   bool            = false
    }
        
    window: WinState {
        renderer:               Option<Renderer<Window>> = None // muh alignment ;-;
        first_focus:            bool            = true
        width:                  int             = 640
        height:                 int             = 480
        title:                  &'static str    = "Worldsong"
        opengl_major_version:   u32             = 3
        opengl_minor_version:   u32             = 0
    }
    
    sim: SimState {
        pause:                  bool            = false
        delta_time:             u64             = 0
        color_r:                u8              = 0
        color_g:                u8              = 0
        color_b:                u8              = 255
    }
)

fn initialize(data: &mut Data) {

    let window = match sdl2::video::Window::new(data.window.title, sdl2::video::PosCentered, sdl2::video::PosCentered, data.window.width, data.window.height, sdl2::video::OPENGL) 
    {
        Ok(window) => window,
        Err(err) => fail!(format!("failed to create window: {}", err))
    };
    
    let renderer = match sdl2::render::Renderer::from_window(window, sdl2::render::DriverAuto, sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => fail!(format!("failed to create renderer: {}", err))
    };
    
    data.window.renderer = Some(renderer);
}