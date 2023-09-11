use iced::widget::scrollable;
#[allow(unused_imports)]
use iced::{window, Command};

use crate::app::{MainApp, MainAppMessage};

pub fn update(app: &mut MainApp, msg: MainAppMessage) -> Command<MainAppMessage> {
    match msg {
        MainAppMessage::ChangeTab(group) => {
            app.tab = group;
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
            return window::close();
        }
        MainAppMessage::SelectSkinTone(t) => app.tone = t,
        MainAppMessage::OnSearchEmoji(s) => app.search = s,
        MainAppMessage::HoverEmoji(n, e, s) => app.emoji_hovered = (n, e, s),
    }

    Command::none()
}
