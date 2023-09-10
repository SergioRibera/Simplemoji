use emojis::{Emoji, Group};
use iced::widget::scrollable::{self, Properties};
use iced::widget::text_input::{Icon, Id};
use iced::widget::{button, pick_list, text_input, Button, Column, Row, Scrollable, Text};
use iced::{Element, Length};
use iced_aw::Grid;
use iced_winit::core::text::Shaping;

use crate::app::MainAppMessage;
use crate::model::SkinTone;
use crate::styles::{get_btn_transparent_style, get_search_style, get_select_style};
use crate::ICON_FONT;

fn render_emoji<'a>(e: &'a Emoji) -> Element<'a, MainAppMessage> {
    button(Text::new(e.as_str()).shaping(Shaping::Advanced))
        .style(get_btn_transparent_style(false))
        .on_press(MainAppMessage::CopyEmoji(e.to_string()))
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
            grid = grid.push(render_emoji(e));
        }
    } else {
        for e in emojis::iter().filter(|e| {
            e.shortcodes()
                .any(|s| s.to_lowercase().contains(&search.to_lowercase()))
        }) {
            let e = e.with_skin_tone((*tone).clone().into()).unwrap_or(e);
            grid = grid.push(render_emoji(e));
        }
    }

    let scroll = Scrollable::new(grid)
        .width(Length::Fill)
        .height(Length::Shrink)
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
    .width(Length::Fixed(40.))
    .into()
}

pub fn show_tab<'a>(
    tabs: &[(Group, String)],
    search: &'a str,
    tone: &'a SkinTone,
    selected: &'a Group,
    on_click: fn(Group) -> MainAppMessage,
) -> Element<'a, MainAppMessage> {
    let mut tab = Row::new()
        .width(Length::Fill)
        .align_items(iced::Alignment::Center);

    for (t, i) in tabs {
        tab = tab.push(
            Button::new(Text::new(i.clone()))
                .style(get_btn_transparent_style(t == selected))
                .on_press(on_click(*t)),
        );
    }

    Column::new()
        .push(tab)
        .push(show_search_row(search, tone))
        .push(render_emoji_grids(search, selected, tone))
        .spacing(10.)
        .width(Length::Fill)
        .into()
}
