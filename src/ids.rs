use iced::widget::{scrollable, text_input};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SEARCH_ID: text_input::Id = text_input::Id::new("search_input");
    pub static ref EMOJI_SCROLL_ID: scrollable::Id = scrollable::Id::new("scrollable_grid");
}
