use iced::border::Radius;
use iced::widget::button;
use iced::{Background, Border, Color};

pub struct TransparentButton(bool);

impl button::StyleSheet for TransparentButton {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            shadow_offset: iced::Vector::default(),
            background: self
                .0
                .then(|| Background::Color(Color::new(1., 1., 1., 0.15))),
            border: Border {
                color: style.palette().primary,
                width: 0.,
                radius: Radius::default(),
            },
            text_color: style.palette().text,
            shadow: iced::Shadow::default(),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let active = self.active(style);

        button::Appearance {
            shadow_offset: active.shadow_offset + iced::Vector::new(0.0, 1.0),
            background: Some(Background::Color(Color::new(1., 1., 1., 0.25))),
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
