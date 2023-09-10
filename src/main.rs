use app::MainApp;
use clap::Parser;
use iced::window::Level;
use iced::{Application, Font, Settings};
use settings::ArgOpts;

mod app;
mod components;
mod model;
mod settings;
mod styles;
mod update;
mod view;

pub const ICON_FONT: Font = Font::with_name("Noto Color Emoji");

fn main() -> iced::Result {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let args = ArgOpts::parse();
    let height = if args.show_preview { 380u32 } else { 320u32 };

    MainApp::run(Settings {
        window: iced::window::Settings {
            size: (325u32, height),
            visible: true,
            resizable: false,
            decorations: true,
            level: Level::AlwaysOnTop,
            ..Default::default()
        },
        flags: args,
        default_text_size: 20.,
        default_font: ICON_FONT,
        ..Default::default()
    })
}
