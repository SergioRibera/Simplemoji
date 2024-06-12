mod emojis_view;
mod preview;
mod search;

pub use emojis_view::*;
pub use preview::*;
pub use search::*;

use emojis::Group;
use iced::widget::{Button, Column, Row};
use iced::{Element, Length};

use crate::app::MainAppMessage;
use crate::components::render_emoji;
use crate::layouts::render_emoji_grids;
use crate::settings::ArgOpts;
use crate::skin_tone::SkinTone;
use crate::styles::get_btn_transparent_style;

pub fn show_content<'a>(
    tabs: &'a [(Group, String)],
    settings: &'a ArgOpts,
    search: &'a str,
    tone: &'a SkinTone,
    emoji_hovered: &'a (String, String, Vec<String>),
    selected: &'a Group,
    on_click: fn(Group) -> MainAppMessage,
) -> Element<'a, MainAppMessage> {
    let mut tab = Row::new()
        .spacing(1.0)
        .width(Length::Fill);

    for (t, i) in tabs {
        tab = tab.push(
            Button::new(render_emoji(i.as_str()))
                .style(get_btn_transparent_style(t == selected))
                .on_press(on_click(*t)),
        );
    }

    let mut col = Column::new().push(tab);
    if settings.show_search {
        col = col.push(show_search_row(search, tone));
    }
    if settings.show_preview {
        col = col.push(show_preview(emoji_hovered));
    }
    col.push(render_emoji_grids(search, selected, tone))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
