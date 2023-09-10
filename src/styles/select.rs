use std::rc::Rc;

use iced::widget::pick_list;
use iced::{overlay, Color};

pub struct SelectStyle;

impl pick_list::StyleSheet for SelectStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> pick_list::Appearance {
        let palette = style.extended_palette();
        let bg = palette.background.weak.color;

        pick_list::Appearance {
            text_color: palette.background.weak.text,
            background: Color::new(bg.r, bg.g, bg.b, 0.6).into(),
            placeholder_color: palette.background.strong.color,
            handle_color: Color::TRANSPARENT,
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: palette.background.strong.color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
        let palette = style.extended_palette();

        pick_list::Appearance {
            background: palette.background.weak.color.into(),
            ..self.active(style)
        }
    }
}

pub struct SelectMenuStyle;

impl overlay::menu::StyleSheet for SelectMenuStyle {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> overlay::menu::Appearance {
        let palette = style.extended_palette();

        overlay::menu::Appearance {
            text_color: palette.background.weak.text,
            background: palette.background.weak.color.into(),
            border_width: 0.0,
            border_radius: 0.0.into(),
            border_color: palette.background.strong.color,
            selected_text_color: palette.primary.strong.text,
            selected_background: palette.primary.strong.color.into(),
        }
    }
}

pub fn get_select_style() -> iced::theme::PickList {
    iced::theme::PickList::Custom(Rc::new(SelectStyle), Rc::new(SelectMenuStyle))
}
