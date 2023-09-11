use iced::subscription::events_with;
use iced::widget::{container, text_input};
use iced::{mouse, Application, Command, Event, Length, Subscription, Theme};

use crate::model::SkinTone;
use crate::settings::ArgOpts;
use crate::update;
use crate::view::show_content;

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
}

impl Application for MainApp {
    type Executor = iced::executor::Default;
    type Message = MainAppMessage;
    type Theme = Theme;
    type Flags = ArgOpts;

    fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
        let tone = settings.tone.unwrap_or_default();
        (
            MainApp {
                settings,
                search: String::new(),
                tone,
                tab: emojis::Group::SmileysAndEmotion,
                emoji_hovered: emojis::Group::SmileysAndEmotion
                    .emojis()
                    .next()
                    .map(|e| (
                        e.name().to_string(),
                        e.to_string(),
                        e.shortcodes().map(|s| s.to_string()).collect::<Vec<String>>()
                    ))
                    .unwrap(),
                theme: match dark_light::detect() {
                    dark_light::Mode::Light => Theme::Light,
                    _ => Theme::Dark,
                },
                tabs: vec![
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
                ],
            },
            text_input::focus(text_input::Id::new("search_input")),
        )
    }

    fn title(&self) -> String {
        "Simplemoji".to_string()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        events_with(|e, _status| match e {
            Event::Mouse(mouse::Event::CursorLeft) => Some(MainAppMessage::HiddeApplication),
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        update::update(self, message)
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        container(show_content(
                self.tabs.as_ref(),
                &self.settings,
                &self.search,
                &self.tone,
                &self.emoji_hovered,
                &self.tab,
                MainAppMessage::ChangeTab,
            ))
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
