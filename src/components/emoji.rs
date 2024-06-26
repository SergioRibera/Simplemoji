use emojis::Emoji;
use iced::widget::{button, MouseArea, Text};
use iced::Element;
use iced_tiny_skia::core::text::Shaping;

use crate::app::MainAppMessage;
use crate::styles::get_btn_transparent_style;

pub fn render_emoji(e: &str) -> Element<'_, MainAppMessage> {
    Text::new(e).shaping(Shaping::Advanced).into()
}

pub fn render_emoji_btn(e: &Emoji) -> Element<'_, MainAppMessage> {
    MouseArea::new(
        button(render_emoji(e.as_str()))
            .style(get_btn_transparent_style(false))
            .on_press(MainAppMessage::CopyEmoji(e.to_string())),
    )
    .on_enter(MainAppMessage::HoverEmoji(
        e.name().to_string(),
        e.to_string(),
        e.shortcodes().map(|s| s.to_string()).collect(),
    ))
    .into()
}
