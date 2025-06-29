use std::borrow::BorrowMut;
use std::rc::Rc;

use arboard::Clipboard;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::settings::ArgOpts;
use crate::skin_tone::SkinTone;
use crate::utils::{emoji_to_model, emojis_from_group, get_default_tabs, group_from};
use crate::{EmojiHandle, EmojiModel, MainWindow, TabsHandle, EMOJI_COLS};

pub struct MainApp {
    window: MainWindow,
    settings: ArgOpts,
    search: SharedString,
    tone: SkinTone,

    content: ModelRc<ModelRc<EmojiModel>>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            window: MainWindow::new().unwrap(),
            settings: Default::default(),
            search: Default::default(),
            tone: Default::default(),
            content: emojis_from_group(emojis::Group::SmileysAndEmotion),
        }
    }
}

/*
TODO:
- focus search input on start
*/

impl MainApp {
    pub fn new(settings: ArgOpts) -> Self {
        let tone = settings.tone.unwrap_or_default();

        Self {
            tone,
            settings,
            ..Default::default()
        }
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        self.window.set_show_preview(self.settings.show_preview);
        let tabs = self.window.global::<TabsHandle>();
        tabs.set_tab(emojis::Group::SmileysAndEmotion as i32);
        tabs.set_tabs(get_default_tabs());

        self.window.set_emojis(self.content.clone());
        tabs.on_change_tab({
            let content = self.content.clone();
            move |id| {
                let group = group_from(id);
                let emojis = group
                    .emojis()
                    .collect::<Vec<_>>()
                    .chunks(EMOJI_COLS)
                    .map(|e| {
                        ModelRc::from(
                            e.iter()
                                .cloned()
                                .map(emoji_to_model)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                    })
                    .collect::<Vec<ModelRc<_>>>();
                content
                    .as_any()
                    .downcast_ref::<VecModel<ModelRc<EmojiModel>>>()
                    .unwrap()
                    .set_vec(emojis);
            }
        });

        self.window.on_close({
            #[cfg(not(debug_assertions))]
            let window = self.window.as_weak();
            move || {
                #[cfg(not(debug_assertions))]
                window
                    .unwrap()
                    .window()
                    .dispatch_event(slint::platform::WindowEvent::CloseRequested);
            }
        });

        self.window.global::<EmojiHandle>().on_click({
            let window = self.window.as_weak();
            let close = self.settings.close_on_copy;
            let cmd = self.settings.copy_command.clone();
            let mut clipboard = cmd.is_none().then(|| Clipboard::new().unwrap());

            move |emoji| {
                if let Some(cmd) = cmd.as_deref() {
                    let mut cmd = cmd.split(' ');
                    let bin = cmd.next().unwrap();
                    let mut args = cmd.collect::<Vec<&str>>();
                    args.push(&emoji);
                    _ = std::process::Command::new(bin)
                        .args(args)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                } else if let Some(clipboard) = clipboard.as_mut() {
                    clipboard.set_text(emoji.as_str()).unwrap();
                }
                if close {
                    window
                        .unwrap()
                        .window()
                        .dispatch_event(slint::platform::WindowEvent::CloseRequested);
                }
            }
        });

        self.window.run()
    }
}
