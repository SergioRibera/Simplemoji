use emojis::{Emoji, Group};
use iced::widget::scrollable::{self, Properties};
use iced::widget::text::Shaping;
use iced::widget::text_input::{Icon, Id};
use iced::widget::{button, pick_list, text_input, Button, Column, Row, Scrollable, Text};
use iced::{Element, Length};
use iced_aw::Grid;

use crate::app::MainAppMessage;
use crate::components::Hoverable;
use crate::model::SkinTone;
use crate::settings::ArgOpts;
use crate::styles::{get_btn_transparent_style, get_search_style, get_select_style};
use crate::ICON_FONT;

fn render_emoji<'a>(e: &'a str) -> Element<'a, MainAppMessage> {
    Text::new(e)
        .shaping(Shaping::Advanced)
        .font(ICON_FONT)
        .into()
}

fn render_emoji_btn<'a>(e: &'a Emoji) -> Element<'a, MainAppMessage> {
    Hoverable::new(
        button(render_emoji(e.as_str()))
            .style(get_btn_transparent_style(false))
            .on_press(MainAppMessage::CopyEmoji(e.to_string()))
            .into(),
    )
    .on_hover(MainAppMessage::HoverEmoji(
        e.name().to_string(),
        e.to_string(),
        e.shortcodes().map(|s| s.to_string()).collect(),
    ))
    .into()
}

pub fn render_emoji_grids<'a>(
    search: &'a str,
    g: &'a Group,
    tone: &'a SkinTone,
) -> Element<'a, MainAppMessage> {
    let mut grid = Grid::new().strategy(iced_aw::grid::Strategy::Columns(9));

    if search.is_empty() {
        for e in g.emojis() {
            let e = e.with_skin_tone((*tone).clone().into()).unwrap_or(e);
            grid.insert(render_emoji_btn(e));
        }
    } else {
        for e in emojis::iter().filter(|e| {
            e.name().to_lowercase().contains(&search.to_lowercase())
                || e.shortcodes()
                    .any(|s| s.to_lowercase().contains(&search.to_lowercase()))
        }) {
            let e = e.with_skin_tone((*tone).clone().into()).unwrap_or(e);
            grid.insert(render_emoji_btn(e));
        }
    }

    let scroll = Scrollable::new(grid)
        .width(Length::Fill)
        .id(scrollable::Id::new("scrollable_grid"))
        .direction(iced::widget::scrollable::Direction::Vertical(
            Properties::default().width(5).scroller_width(5),
        ));

    scroll.into()
}

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
                .id(Id::new("search_input"))
                .style(get_search_style())
                .on_input(MainAppMessage::OnSearchEmoji),
        )
        .push(show_tone_selector(tone))
        .spacing(10)
        .width(Length::Fill)
        .padding(10)
        .into()
}

pub fn show_tone_selector(tone: &SkinTone) -> Element<'_, MainAppMessage> {
    pick_list(
        SkinTone::get_all(),
        Some(tone.clone()),
        MainAppMessage::SelectSkinTone,
    )
    .style(get_select_style())
    .text_shaping(Shaping::Advanced)
    .width(Length::Fixed(35.))
    .into()
}

pub fn show_preview<'a>(
    selected: &'a (String, String, Vec<String>),
) -> Element<'a, MainAppMessage> {
    let mut info = Column::new()
        .push(Text::new(selected.0.as_str()).size(18))
        .width(Length::Fill)
        .spacing(5);

    let mut shortcodes = Row::new()
        .spacing(5)
        .height(Length::Fill)
        .width(Length::Fill);

    for s in &selected.2 {
        shortcodes = shortcodes.push(Text::new(s).size(12));
    }

    info = info.push(shortcodes);

    Row::new()
        .push(
            Text::new(selected.1.as_str())
                .font(ICON_FONT)
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
        .width(Length::Fill)
        .align_items(iced::Alignment::Center);

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
        .into()
}
