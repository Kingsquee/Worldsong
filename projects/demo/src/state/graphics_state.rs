extern crate sdl2;
use self::sdl2::Sdl;
use self::sdl2::render::Renderer;

data! {
    GraphicsState {
        title:                  String                  = "Worldsong".to_string()
        width:                  u32                     = 640u32
        height:                 u32                     = 480u32
        sdl:                    Sdl                     = sdl2::init().everything().unwrap()
        renderer:               Renderer<'static>       = init_renderer(&title, &sdl, width, height)
        first_focus:            bool                    = true
        opengl_major_version:   u32                     = 3
        opengl_minor_version:   u32                     = 0
    }
}

pub fn init_renderer(title: &str, sdl: &Sdl, width: u32, height: u32) -> Renderer<'static> {
    let window = sdl.window(title, width, height,)
                .position_centered()
                .opengl()
                .build()
                .unwrap();
    let renderer = window.renderer().build().unwrap();
    renderer
}
