use app::MainApp;
use iced::window::Level;
use iced::{Application, Font, Settings};

mod app;
mod model;
mod styles;
mod update;
mod view;

pub const ICON_FONT: Font = Font::with_name("Noto Color Emoji");

fn main() -> iced::Result {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (325u32, 320u32),
            visible: true,
            resizable: false,
            decorations: true,
            level: Level::AlwaysOnTop,
            // transparent: settings.transparent(),
            ..Default::default()
        },
        default_text_size: 20.,
        default_font: ICON_FONT,
        ..Default::default()
    })
}
