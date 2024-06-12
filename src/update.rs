use iced::widget::scrollable;
#[allow(unused_imports)]
use iced::{window, Command};

use crate::app::{MainApp, MainAppMessage};
use crate::ids::EMOJI_SCROLL_ID;

pub fn update(app: &mut MainApp, msg: MainAppMessage) -> Command<MainAppMessage> {
    match msg {
        MainAppMessage::ChangeTab(group) => {
            app.tab = group;
            return scrollable::scroll_to(
                EMOJI_SCROLL_ID.clone(),
                scrollable::AbsoluteOffset { x: 0., y: 0. },
            );
        }
        MainAppMessage::HiddeApplication => {
            #[cfg(not(debug_assertions))]
            return window::close(iced::window::Id::MAIN);
        }
        MainAppMessage::CopyEmoji(emoji) => {
            if let Some(cmd) = app.settings.copy_command.as_deref() {
                let mut cmd = cmd.split(' ');
                let bin = cmd.next().unwrap();
                let mut args = cmd.collect::<Vec<&str>>();
                args.push(&emoji);
                std::process::Command::new(bin).args(args).spawn().unwrap();
            } else {
                app.clipboard.set_text(emoji).unwrap()
            }
        }
        MainAppMessage::SelectSkinTone(t) => app.tone = t,
        MainAppMessage::OnSearchEmoji(s) => app.search = s,
        MainAppMessage::HoverEmoji(n, e, s) => app.emoji_hovered = (n, e, s),
    }

    Command::none()
}
