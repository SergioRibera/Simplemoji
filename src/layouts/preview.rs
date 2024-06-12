use iced::widget::{Column, Row, Text};
use iced::{Element, Font, Length};
use iced_tiny_skia::core::text::Shaping;

use crate::app::MainAppMessage;

pub fn show_preview(selected: &(String, String, Vec<String>)) -> Element<'_, MainAppMessage> {
    let mut info = Column::new()
        .push(Text::new(selected.0.as_str()).font(Font::DEFAULT).size(18))
        .width(Length::Fill)
        .spacing(5);

    let mut shortcodes = Row::new()
        .spacing(5)
        .height(Length::Fill)
        .width(Length::Fill);

    for s in &selected.2 {
        shortcodes = shortcodes.push(Text::new(s).font(Font::DEFAULT).size(12));
    }

    info = info.push(shortcodes);

    Row::new()
        .push(
            Text::new(selected.1.as_str())
                .shaping(Shaping::Advanced)
                .size(40),
        )
        .push(info)
        .align_items(iced::Alignment::Center)
        .height(Length::Fixed(70.))
        .padding([0, 10])
        .spacing(10)
        .into()
}
