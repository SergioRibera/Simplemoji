use app::MainApp;
use clap::Parser;
use iced::window::Level;
use iced::{Application, Font, Point, Settings, Size};
use lazy_static::lazy_static;
use settings::ArgOpts;
use utils::mouse_to_window_pos;

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

    use mouse_position::MouseExt;
    let position = mouse_position::Mouse::default()
        .get_pos()
        .map(|pos| {
            let (x, y) = mouse_to_window_pos(pos);
            iced::window::Position::Specific(Point::new(x as f32, y as f32))
        })
        .unwrap_or(iced::window::Position::Centered);

    MainApp::run(Settings {
        window: iced::window::Settings {
            position,
            size: Size::new(APP_WIDTH as f32, *APP_HEIGHT as f32),
            visible: true,
            resizable: false,
            decorations: false,
            level: Level::AlwaysOnTop,
            ..Default::default()
        },
        flags: args,
        default_text_size: iced::Pixels(20.),
        ..Default::default()
    })
}
