use arboard::Clipboard;
use iced::widget::{container, text_input};
use iced::{mouse, Application, Command, Event, Length, Subscription, Theme};

use crate::color::ToRgba;
use crate::ids::SEARCH_ID;
use crate::layouts::show_content;
use crate::settings::ArgOpts;
use crate::skin_tone::SkinTone;
use crate::update;
use crate::utils::get_default_tabs;

#[derive(Clone, Debug)]
pub enum MainAppMessage {
    HiddeApplication,
    ChangeTab(emojis::Group),
    CopyEmoji(String),
    SelectSkinTone(SkinTone),
    OnSearchEmoji(String),
    HoverEmoji(String, String, Vec<String>),
}

pub struct MainApp {
    pub tabs: Vec<(emojis::Group, String)>,
    pub settings: ArgOpts,
    pub search: String,
    pub emoji_hovered: (String, String, Vec<String>),
    pub tone: SkinTone,
    pub tab: emojis::Group,
    pub theme: Theme,
    pub clipboard: Clipboard,
}

impl MainApp {
    pub fn new(settings: ArgOpts) -> Self {
        let tone = settings.tone.unwrap_or_default();
        let theme = match dark_light::detect() {
            dark_light::Mode::Light => Theme::Light,
            _ => Theme::Dark,
        };
        let theme = Theme::custom(
            "Custom".to_owned(),
            iced::theme::Palette {
                background: settings
                    .background_color
                    .as_deref()
                    .inspect(|v| log::debug!("Background Custom Color: {v}"))
                    .map(|b| b.to_rgba().unwrap())
                    .unwrap_or(theme.palette().background),
                primary: settings
                    .primary_color
                    .as_deref()
                    .inspect(|v| log::debug!("Primary Custom Color: {v}"))
                    .map(|b| b.to_rgba().unwrap())
                    .unwrap_or(theme.palette().primary),
                ..theme.palette()
            },
        );

        Self {
            tone,
            theme,
            settings,
            ..Default::default()
        }
    }
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            tabs: get_default_tabs(),
            settings: Default::default(),
            search: Default::default(),
            tone: Default::default(),
            tab: emojis::Group::SmileysAndEmotion,
            clipboard: Clipboard::new().unwrap(),
            emoji_hovered: emojis::Group::SmileysAndEmotion
                .emojis()
                .next()
                .map(|e| {
                    (
                        e.name().to_string(),
                        e.to_string(),
                        e.shortcodes()
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    )
                })
                .unwrap(),
            theme: match dark_light::detect() {
                dark_light::Mode::Light => Theme::Light,
                _ => Theme::Dark,
            },
        }
    }
}

impl Application for MainApp {
    type Executor = iced::executor::Default;
    type Message = MainAppMessage;
    type Theme = Theme;
    type Flags = ArgOpts;

    fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
        let focus_search = if settings.show_search {
            text_input::focus(SEARCH_ID.clone())
        } else {
            Command::none()
        };

        (MainApp::new(settings), focus_search)
    }

    fn title(&self) -> String {
        "Simplemoji".to_string()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        iced::event::listen_with(|e, _status| match e {
            Event::Mouse(mouse::Event::CursorLeft) => Some(MainAppMessage::HiddeApplication),
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        update::update(self, message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme> {
        container(show_content(
            self.tabs.as_ref(),
            &self.settings,
            &self.search,
            &self.tone,
            &self.emoji_hovered,
            &self.tab,
            MainAppMessage::ChangeTab,
        ))
        .padding(5)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}
