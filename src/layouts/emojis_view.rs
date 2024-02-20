use emojis::Group;
use iced::widget::scrollable::Properties;
use iced::widget::Scrollable;
use iced::{Element, Length};
use iced_aw::{Grid, GridRow};

use crate::app::MainAppMessage;
use crate::components::render_emoji_btn;
use crate::ids::EMOJI_SCROLL_ID;
use crate::skin_tone::SkinTone;

pub fn render_emoji_grids<'a>(
    search: &'a str,
    g: &'a Group,
    tone: &'a SkinTone,
) -> Element<'a, MainAppMessage> {
    let r = if search.is_empty() {
        g.emojis().collect::<Vec<_>>()
    } else {
        emojis::iter()
            .filter(|e| {
                e.name().to_lowercase().contains(&search.to_lowercase())
                    || e.shortcodes()
                        .any(|s| s.to_lowercase().contains(&search.to_lowercase()))
            })
            .collect::<Vec<_>>()
    };

    let mut rows = Vec::new();

    for e in r.chunks(9) {
        rows.push(GridRow::with_elements(
            e.iter()
                .map(|e| render_emoji_btn(e.with_skin_tone((*tone).into()).unwrap_or(e)))
                .collect(),
        ));
    }

    let scroll = Scrollable::new(Grid::with_rows(rows))
        .width(Length::Fill)
        .id(EMOJI_SCROLL_ID.clone())
        .direction(iced::widget::scrollable::Direction::Vertical(
            Properties::default().width(5).scroller_width(5),
        ));

    scroll.into()
}
