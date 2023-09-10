use iced::widget::text_input;

pub struct SearchStyle;

impl text_input::StyleSheet for SearchStyle {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 0.,
            border_color: palette.background.strong.color,
            icon_color: palette.background.strong.color,
        }
    }

    fn hovered(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 0.,
            border_color: palette.background.base.text,
            icon_color: palette.background.base.text,
        }
    }

    fn focused(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 0.,
            border_color: palette.primary.strong.color,
            icon_color: palette.primary.strong.color,
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        palette.background.strong.color
    }

    fn value_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        palette.primary.strong.text
    }

    fn selection_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        palette.primary.weak.color
    }

    fn disabled(&self, style: &Self::Style) -> text_input::Appearance {
        let palette = style.extended_palette();

        text_input::Appearance {
            background: palette.background.base.color.into(),
            border_radius: 2.0.into(),
            border_width: 0.,
            border_color: palette.background.base.text,
            icon_color: palette.background.base.text,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> iced::Color {
        let palette = style.extended_palette();

        let [r, g, b, a] = palette.primary.base.text.into_linear();
        iced::Color::new(r, g, b, a / 2.)
    }
}

pub fn get_search_style() -> iced::theme::TextInput {
    iced::theme::TextInput::Custom(Box::new(SearchStyle))
}
