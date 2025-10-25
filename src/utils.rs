use std::rc::Rc;

use display_info::DisplayInfo;
use slint::{ModelRc, SharedString, VecModel};

use crate::{APP_MOUSE_MARGIN, EMOJI_COLS};
use ui::{EmojiModel, emojis};

pub fn mouse_to_window_pos((app_width, app_height): (i32, i32), (x, y): (i32, i32)) -> (i32, i32) {
    let Ok(DisplayInfo {
        width,
        height,
        x: display_x,
        y: display_y,
        ..
    }) = DisplayInfo::from_point(x, y)
    else {
        return (x, y);
    };

    let left_x = ((x - display_x) as u32) < (width / 2);
    let bottom_y = ((y - display_y) as u32) > (height / 2);

    match (left_x, bottom_y) {
        // Top Left
        (true, false) => (x - APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Top Right
        (false, false) => ((x - app_width) + APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Bottom Left
        (true, true) => (x - APP_MOUSE_MARGIN, y - app_height + APP_MOUSE_MARGIN),
        // Bottom Right
        (false, true) => (
            (x - app_width) + APP_MOUSE_MARGIN,
            y - app_height + APP_MOUSE_MARGIN,
        ),
    }
}

pub fn group_from(i: i32) -> emojis::Group {
    match i {
        1 => emojis::Group::PeopleAndBody,
        2 => emojis::Group::AnimalsAndNature,
        3 => emojis::Group::FoodAndDrink,
        4 => emojis::Group::TravelAndPlaces,
        5 => emojis::Group::Activities,
        6 => emojis::Group::Objects,
        7 => emojis::Group::Symbols,
        8 => emojis::Group::Flags,
        _ => emojis::Group::SmileysAndEmotion,
    }
}

pub fn emoji_from_model(e: &EmojiModel) -> &'static emojis::Emoji {
    emojis::get(&e.character).unwrap()
}

pub fn emoji_to_model(e: &'static emojis::Emoji) -> EmojiModel {
    EmojiModel {
        character: e.as_str().into(),
        codes: Rc::new(
            e.shortcodes()
                .map(SharedString::from)
                .collect::<VecModel<_>>(),
        )
        .into(),
        name: e.name().into(),
    }
}

pub fn emojis_to_modelrc(
    e: impl IntoIterator<Item = &'static emojis::Emoji>,
) -> ModelRc<EmojiModel> {
    let emojis = e.into_iter().map(emoji_to_model).collect::<Vec<_>>();

    ModelRc::from(emojis.as_slice())
}

pub fn emojis_from_group(g: emojis::Group) -> Vec<Vec<EmojiModel>> {
    g.emojis()
        .map(emoji_to_model)
        .collect::<Vec<_>>()
        .chunks(EMOJI_COLS)
        .map(<[ui::EmojiModel]>::to_vec)
        .collect::<Vec<_>>()
}

pub fn get_default_tabs(tone: emojis::SkinTone) -> ModelRc<EmojiModel> {
    let map = |group: emojis::Group| {
        group
            .emojis()
            .next()
            .map(|e| {
                let e = e.with_skin_tone(tone).unwrap_or(e);
                emoji_to_model(e)
            })
            .unwrap()
    };

    VecModel::from_slice(&[
        map(emojis::Group::SmileysAndEmotion),
        map(emojis::Group::PeopleAndBody),
        map(emojis::Group::AnimalsAndNature),
        map(emojis::Group::FoodAndDrink),
        map(emojis::Group::TravelAndPlaces),
        map(emojis::Group::Objects),
        map(emojis::Group::Activities),
        map(emojis::Group::Symbols),
        map(emojis::Group::Flags),
    ])
}
