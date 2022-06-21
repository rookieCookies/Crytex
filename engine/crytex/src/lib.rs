use crytex_render::RenderHandle;

pub struct App;

impl App {
    pub fn init() {
        let _renderer = pollster::block_on(RenderHandle::init());
    }
}
pub struct AppBuilder;