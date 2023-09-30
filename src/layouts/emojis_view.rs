use emojis::Group;
use iced::widget::scrollable::Properties;
use iced::widget::Scrollable;
use iced::{Element, Length};
use iced_aw::Grid;

use crate::app::MainAppMessage;
use crate::components::render_emoji_btn;
use crate::ids::EMOJI_SCROLL_ID;
use crate::skin_tone::SkinTone;

pub fn render_emoji_grids<'a>(
    search: &'a str,
    g: &'a Group,
    tone: &'a SkinTone,
) -> Element<'a, MainAppMessage> {
    let mut grid = Grid::new().strategy(iced_aw::grid::Strategy::Columns(9));

    if search.is_empty() {
        for e in g.emojis() {
            let e = e.with_skin_tone((*tone).into()).unwrap_or(e);
            grid.insert(render_emoji_btn(e));
        }
    } else {
        for e in emojis::iter().filter(|e| {
            e.name().to_lowercase().contains(&search.to_lowercase())
                || e.shortcodes()
                    .any(|s| s.to_lowercase().contains(&search.to_lowercase()))
        }) {
            let e = e.with_skin_tone((*tone).into()).unwrap_or(e);
            grid.insert(render_emoji_btn(e));
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
