use iced::subscription::events_with;
use iced::widget::{container, scrollable, text_input, Column};
use iced::{mouse, window, Application, Command, Event, Length, Subscription, Theme};

use crate::model::SkinTone;
use crate::settings::ArgOpts;
use crate::view::show_content;

#[derive(Clone, Debug)]
pub enum MainAppMessage {
    HiddeApplication,
    ChangeTab(emojis::Group),
    CopyEmoji(String),
    SelectSkinTone(SkinTone),
    OnSearchEmoji(String),
}

pub struct MainApp {
    tabs: Vec<(emojis::Group, String)>,
    settings: ArgOpts,
    search: String,
    tone: SkinTone,
    tab: emojis::Group,
    theme: Theme,
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
        // crate::subscription::manage_emojis(self.tab.clone())
        events_with(|e, _status| match e {
            Event::Mouse(mouse::Event::CursorLeft) => Some(MainAppMessage::HiddeApplication),
            _ => None,
        })
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MainAppMessage::ChangeTab(group) => {
                self.tab = group;
                return scrollable::scroll_to(
                    scrollable::Id::new("scrollable_grid"),
                    scrollable::AbsoluteOffset { x: 0., y: 0. },
                );
            }
            MainAppMessage::CopyEmoji(emoji) => {
                return iced::clipboard::write(emoji);
            }
            MainAppMessage::HiddeApplication => {
                #[cfg(not(debug_assertions))]
                return window::change_mode(window::Mode::Hidden);
            }
            MainAppMessage::SelectSkinTone(t) => self.tone = t,
            MainAppMessage::OnSearchEmoji(s) => self.search = s,
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let col = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(10)
            .push(show_content(
                self.tabs.as_ref(),
                &self.settings,
                self.search.as_str(),
                &self.tone,
                &self.tab,
                MainAppMessage::ChangeTab,
            ));

        container(col)
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