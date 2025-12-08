#![cfg_attr(
    all(target_os = "windows", not(debug_assertions),),
    windows_subsystem = "windows"
)]

use std::sync::Arc;

use app::MainApp;
use clap::Parser;
use settings::ArgOpts;
use slint::ComponentHandle;
use slint::winit_030::WinitWindowAccessor;
use slint::winit_030::winit::dpi::{LogicalPosition, LogicalSize, Position, Size};
use slint::winit_030::winit::window::WindowButtons;

use self::utils::mouse_to_window_pos;

mod app;
mod app_data;
mod color;
mod navigation;
mod recents;
mod settings;
mod utils;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const TOLERANCE: i_slint_core::Coord = 0.001;
pub const APP_WIDTH: i32 = 315;
pub const APP_MOUSE_MARGIN: i32 = 25;
pub const EMOJI_COLS: usize = 9;

fn main() -> Result<(), slint::PlatformError> {
    env_logger::Builder::from_env("SIMPLEMOJI").init();

    let mut flags = ArgOpts::parse();
    flags.ime = imekit::InputMethod::new()
        .inspect_err(|e| log::error!("Fail to create InputMethod: {e}"))
        .ok()
        .map(Arc::new);

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
                .with_inner_size(Size::Logical(LogicalSize::new(
                    APP_WIDTH as f64,
                    height as f64,
                )))
        })
        .select()?;

    let mut app = MainApp::new(flags);

    slint::invoke_from_event_loop({
        let window = app.window();
        move || {
            slint::spawn_local({
                let window = window.clone();
                async move {
                    window
                        .unwrap()
                        .window()
                        .winit_window()
                        .await
                        .unwrap()
                        .set_ime_allowed(false);
                }
            })
            .unwrap();
            window.unwrap().window().show().unwrap();
        }
    })
    .unwrap();

    app.set_globals();
    app.set_events();
    app.run()
}
