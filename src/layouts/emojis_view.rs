use emojis::Group;
use iced::widget::scrollable::Properties;
use iced::widget::Scrollable;
use iced::{Element, Length};
use iced_aw::{Grid, GridRow};

use crate::app::MainAppMessage;
use crate::components::render_emoji_btn;
use crate::ids::EMOJI_SCROLL_ID;
use crate::skin_tone::SkinTone;
use crate::EMOJI_COLS;

pub fn render_emoji_grids<'a>(
    search: &'a str,
    g: &'a Group,
    tone: &'a SkinTone,
) -> Element<'a, MainAppMessage> {
    let mut grid = Grid::new().column_width(EMOJI_COLS as f32);

    if search.is_empty() {
        let emojis = g.emojis().collect::<Vec<_>>();
        let emojis = emojis.as_slice();
        for chunk in emojis.chunks(EMOJI_COLS) {
            let mut row = GridRow::new();
            for e in chunk {
                let e = e.with_skin_tone((*tone).into()).unwrap_or(e);
                row = row.push(render_emoji_btn(e));
            }
            grid = grid.push(row);
        }
    } else {
        let emojis = emojis::iter()
            .filter(|e| {
                e.name().to_lowercase().contains(&search.to_lowercase())
                    || e.shortcodes()
                        .any(|s| s.to_lowercase().contains(&search.to_lowercase()))
            })
            .collect::<Vec<_>>();
        let emojis = emojis.as_slice();
        for chunk in emojis.chunks(EMOJI_COLS) {
            let mut row = GridRow::new();
            for e in chunk {
                let e = e.with_skin_tone((*tone).into()).unwrap_or(e);
                row = row.push(render_emoji_btn(e));
            }
            grid = grid.push(row);
        }
    }

    let scroll = Scrollable::new(grid)
        .width(Length::Fill)
        .id(EMOJI_SCROLL_ID.clone())
        .direction(iced::widget::scrollable::Direction::Vertical(
            Properties::default().width(5).scroller_width(5),
        ));

    scroll.into()
}
