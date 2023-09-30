use app::MainApp;
use clap::Parser;
use iced::window::Level;
use iced::{Application, Font, Settings};
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
pub static mut APP_HEIGHT: i32 = 320;

fn main() -> iced::Result {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let args = ArgOpts::parse();

    if args.show_preview {
        unsafe {
            APP_HEIGHT = 380;
        }
    }

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (APP_WIDTH as u32, unsafe { APP_HEIGHT } as u32),
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
