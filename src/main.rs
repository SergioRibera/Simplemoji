#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use app::MainApp;
use clap::Parser;
use settings::ArgOpts;
use slint::winit_030::winit::dpi::{LogicalPosition, Position};
use slint::winit_030::winit::window::WindowButtons;
use slint::ComponentHandle;

use self::utils::mouse_to_window_pos;

mod app;
mod color;
mod navigation;
mod settings;
mod utils;

const TOLERANCE: i_slint_core::Coord = 0.001;
pub const APP_WIDTH: i32 = 315;
pub const APP_MOUSE_MARGIN: i32 = 25;
pub const EMOJI_COLS: usize = 9;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let flags = ArgOpts::parse();

    let show_preview = flags.show_preview;
    slint::BackendSelector::new()
        .with_winit_window_attributes_hook(move |attr| {
            let height = if show_preview { 390 } else { 330 };
            let device_state = device_query::DeviceState::new();
            let pos = device_state.query_pointer().coords;
            let (x, y) = mouse_to_window_pos((APP_WIDTH, height), pos);

            attr.with_active(true)
                .with_decorations(false)
                .with_resizable(false)
                .with_visible(true)
                .with_enabled_buttons(WindowButtons::empty())
                .with_position(Position::Logical(LogicalPosition::new(x as f64, y as f64)))
        })
        .select()?;

    let app = MainApp::new(flags);

    slint::invoke_from_event_loop({
        let window = app.window();
        move || {
            window.unwrap().window().show().unwrap();
        }
    })
    .unwrap();

    app.set_globals();
    app.set_events();
    app.run()
}
