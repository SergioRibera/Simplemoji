use std::sync::LazyLock;

use iced::widget::{scrollable, text_input};

pub static SEARCH_ID: LazyLock<text_input::Id> =
    LazyLock::new(|| text_input::Id::new("search_input"));
pub static EMOJI_SCROLL_ID: LazyLock<scrollable::Id> =
    LazyLock::new(|| scrollable::Id::new("scrollable_grid"));
