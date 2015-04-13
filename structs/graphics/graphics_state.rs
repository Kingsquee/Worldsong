extern crate sdl2;
use self::sdl2::Sdl;
use self::sdl2::render::Renderer;

data! {
    GraphicsState {
        title:                  String                  = "Worldsong".to_string()
        width:                  i32                     = 640i32
        height:                 i32                     = 480i32
        sdl:                    Sdl                     = sdl2::init(sdl2::INIT_EVERYTHING).unwrap()
        renderer:               Renderer<'static>       = init_renderer(&title, &sdl, width, height)
        first_focus:            bool                    = true
        opengl_major_version:   u32                     = 3
        opengl_minor_version:   u32                     = 0
    }
}

pub fn init_renderer(title: &str, sdl: &Sdl, width: i32, height: i32) -> Renderer<'static> {
    let window = sdl2::video::Window::new(
                    sdl,
                    title,
                    sdl2::video::WindowPos::PosCentered,
                    sdl2::video::WindowPos::PosCentered,
                    width,
                    height,
                    sdl2::video::OPENGL
                ).unwrap();
    let renderer = sdl2::render::Renderer::from_window(window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED).unwrap();
    renderer
}
