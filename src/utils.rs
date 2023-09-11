use display_info::DisplayInfo;

use crate::{APP_HEIGHT, APP_MOUSE_MARGIN, APP_WIDTH};

pub fn mouse_to_window_pos((x, y): (i32, i32)) -> (i32, i32) {
    let Ok(DisplayInfo {width, height, x: display_x, y: display_y, .. }) = DisplayInfo::from_point(x, y) else { return (x, y); };
    let app_height = unsafe { APP_HEIGHT };

    let left_x = ((x - display_x) as u32) < (width / 2);
    let bottom_y = ((y - display_y) as u32) > (height / 2);

    match (left_x, bottom_y) {
        // Top Left
        (true, false) => (x - APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Top Right
        (false, false) => ((x - APP_WIDTH) + APP_MOUSE_MARGIN, y - APP_MOUSE_MARGIN),
        // Bottom Left
        (true, true) => (x - APP_MOUSE_MARGIN, y - app_height + APP_MOUSE_MARGIN),
        // Bottom Right
        (false, true) => (
            (x - APP_WIDTH) + APP_MOUSE_MARGIN,
            y - app_height + APP_MOUSE_MARGIN,
        ),
    }
}

pub fn get_default_tabs() -> Vec<(emojis::Group, String)> {
    vec![
        emojis::Group::SmileysAndEmotion
            .emojis()
            .next()
            .map(|e| (emojis::Group::SmileysAndEmotion, e.to_string()))
            .unwrap(),
        emojis::Group::PeopleAndBody
            .emojis()
            .next()
            .map(|e| (emojis::Group::PeopleAndBody, e.to_string()))
            .unwrap(),
        emojis::Group::AnimalsAndNature
            .emojis()
            .next()
            .map(|e| (emojis::Group::AnimalsAndNature, e.to_string()))
            .unwrap(),
        emojis::Group::FoodAndDrink
            .emojis()
            .next()
            .map(|e| (emojis::Group::FoodAndDrink, e.to_string()))
            .unwrap(),
        emojis::Group::TravelAndPlaces
            .emojis()
            .next()
            .map(|e| (emojis::Group::TravelAndPlaces, e.to_string()))
            .unwrap(),
        emojis::Group::Objects
            .emojis()
            .next()
            .map(|e| (emojis::Group::Objects, e.to_string()))
            .unwrap(),
        emojis::Group::Activities
            .emojis()
            .next()
            .map(|e| (emojis::Group::Activities, e.to_string()))
            .unwrap(),
        emojis::Group::Symbols
            .emojis()
            .next()
            .map(|e| (emojis::Group::Symbols, e.to_string()))
            .unwrap(),
        emojis::Group::Flags
            .emojis()
            .next()
            .map(|e| (emojis::Group::Flags, e.to_string()))
            .unwrap(),
    ]
}
