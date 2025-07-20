#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use app::MainApp;
use clap::Parser;
use settings::ArgOpts;

mod app;
mod color;
mod navigation;
mod settings;
mod skin_tone;
mod utils;

slint::include_modules!();

const TOLERANCE: i_slint_core::Coord = 0.001;
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
