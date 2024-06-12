use iced::border::Radius;
use iced::widget::button;
use iced::{Background, Border};

pub struct TransparentButton(bool);

impl button::StyleSheet for TransparentButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let palette = style.extended_palette();

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: self
                .0
                .then(|| Background::Color(palette.background.weak.color)),
            border: Border {
                color: palette.background.strong.color,
                width: 0.,
                radius: Radius::default(),
            },
            text_color: palette.primary.strong.text,
            shadow: iced::Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);
        let palette = style.extended_palette();
        let bg = palette.background.weak.color;

        button::Appearance {
            background: Some(Background::Color(bg)),
            ..active
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            ..self.active(style)
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: iced::Vector::default(),
            text_color: iced::Color {
                a: active.text_color.a * 0.5,
                ..active.text_color
            },
            ..active
        }
    }
}

pub fn get_btn_transparent_style(selected: bool) -> iced::theme::Button {
    iced::theme::Button::Custom(Box::new(TransparentButton(selected)))
}
