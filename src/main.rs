use app::MainApp;
use clap::Parser;
use settings::ArgOpts;
// use utils::mouse_to_window_pos;

mod app;
// mod color;
// mod components;
// mod ids;
// mod layouts;
mod settings;
mod skin_tone;
// mod styles;
// mod update;
mod utils;

slint::include_modules!();

pub const APP_MOUSE_MARGIN: i32 = 25;
pub const EMOJI_COLS: usize = 9;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let flags = ArgOpts::parse();

    let device_state = device_query::DeviceState::new();
    let pos = device_state.query_pointer().coords;
    // let (x, y) = mouse_to_window_pos(pos);

    MainApp::new(flags).run()
}
