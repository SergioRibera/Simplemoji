use iced::widget::text_input::Icon;
use iced::widget::{text_input, Row};
use iced::{Element, Length};

use crate::app::MainAppMessage;
use crate::components::show_tone_selector;
use crate::ids::SEARCH_ID;
use crate::skin_tone::SkinTone;
use crate::styles::get_search_style;
use crate::ICON_FONT;

pub fn show_search_row<'a>(s: &'a str, tone: &'a SkinTone) -> Element<'a, MainAppMessage> {
    Row::new()
        .push(
            text_input("Search", s)
                .icon(Icon {
                    font: ICON_FONT,
                    code_point: '🔎',
                    size: Some(20.),
                    spacing: 5.,
                    side: text_input::Side::Left,
                })
                .id(SEARCH_ID.clone())
                .style(get_search_style())
                .on_input(MainAppMessage::OnSearchEmoji),
        )
        .push(show_tone_selector(tone))
        .spacing(10)
        .width(Length::Fill)
        .padding(10)
        .into()
}
