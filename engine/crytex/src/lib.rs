use crytex_log::{LOGGER};

pub struct App;

impl App {
    pub fn init() {
        LOGGER.info("Initialising Crytex Engine")
    }
}
pub struct AppBuilder;