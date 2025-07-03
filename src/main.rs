use app::MainApp;
use clap::Parser;
use settings::ArgOpts;

mod app;
mod color;
mod settings;
mod skin_tone;
mod utils;

slint::include_modules!();

pub const APP_MOUSE_MARGIN: i32 = 25;
pub const EMOJI_COLS: usize = 9;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let flags = ArgOpts::parse();
    let app = MainApp::new(flags);

    app.set_globals();
    app.set_events();
    app.run()
}
