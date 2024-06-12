use app::MainApp;
use clap::Parser;
use iced::window::Level;
use iced::{Application, Font, Settings};
use lazy_static::lazy_static;
use settings::ArgOpts;
use utils::mouse_to_window_pos;

mod app;
mod color;
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
pub const APP_WIDTH: i32 = 320;
pub const EMOJI_COLS: usize = 9;

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

    let flags = ArgOpts::parse();

    let device_state = device_query::DeviceState::new();
    let pos = device_state.query_pointer().coords;
    let (x, y) = mouse_to_window_pos(pos);

    MainApp::run(Settings {
        window: iced::window::Settings {
            position: iced::window::Position::Specific(iced::Point::new(x as f32, y as f32)),
            size: [APP_WIDTH as f32, *APP_HEIGHT as f32].into(),
            visible: true,
            resizable: false,
            decorations: false,
            level: Level::AlwaysOnTop,
            ..Default::default()
        },
        flags,
        default_text_size: 20f32.into(),
        default_font: ICON_FONT,
        ..Default::default()
    })
}
