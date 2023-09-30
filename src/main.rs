use app::MainApp;
use clap::Parser;
use iced::window::Level;
use iced::{Application, Font, Settings};
use lazy_static::lazy_static;
use settings::ArgOpts;

mod app;
mod components;
mod ids;
mod layouts;
mod settings;
mod skin_tone;
mod styles;
mod update;
mod utils;

pub const ICON_FONT: Font = Font::with_name("Noto Color Emoji");
pub const APP_MOUSE_MARGIN: i32 = 25;

pub static APP_WIDTH: i32 = 325;
lazy_static! {
    pub static ref APP_HEIGHT: i32 = {
        let show_preview = std::env::args().any(|a| a.contains("show-preview"));
        if show_preview {
            return 380;
        }
        320
    };
}

fn main() -> iced::Result {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let args = ArgOpts::parse();

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (APP_WIDTH as u32, *APP_HEIGHT as u32),
            visible: true,
            resizable: false,
            decorations: true,
            level: Level::AlwaysOnTop,
            ..Default::default()
        },
        flags: args,
        default_text_size: 20.,
        ..Default::default()
    })
}
