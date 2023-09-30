use iced::widget::pick_list;
use iced::{Element, Length};
use iced_tiny_skia::core::text::Shaping;

use crate::app::MainAppMessage;
use crate::skin_tone::SkinTone;
use crate::styles::get_select_style;

pub fn show_tone_selector(tone: &SkinTone) -> Element<'_, MainAppMessage> {
    pick_list(
        SkinTone::get_all(),
        Some(*tone),
        MainAppMessage::SelectSkinTone,
    )
    .style(get_select_style())
    .text_shaping(Shaping::Advanced)
    .width(Length::Fixed(35.))
    .into()
}
