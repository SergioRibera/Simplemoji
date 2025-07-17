use std::cell::RefCell;
use std::rc::Rc;

use arboard::Clipboard;

use slint::winit_030::winit::event::WindowEvent;
use slint::winit_030::{WinitWindowAccessor, WinitWindowEventResult};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::color::ToRgba;
use crate::navigation::MoveFocus;
use crate::settings::ArgOpts;
use crate::utils::{
    emoji_from_model, emoji_to_model, emojis_from_group, get_default_tabs, group_from,
    mouse_to_window_pos,
};
use crate::{
    EmojiHandle, EmojiModel, MainState, MainWindow, MyColors, Navigation, SearchGlobal, SkinTone,
    TabsHandle, EMOJI_COLS,
};

pub struct MainApp {
    window: MainWindow,
    settings: ArgOpts,
    tone: Rc<RefCell<SkinTone>>,

    content: ModelRc<ModelRc<EmojiModel>>,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            window: MainWindow::new().unwrap(),
            settings: Default::default(),
            tone: Default::default(),
            content: emojis_from_group(emojis::Group::SmileysAndEmotion),
        }
    }
}

/*
TODO:
- keyboard navigation
*/

impl MainApp {
    fn close(_window: Weak<MainWindow>) {
        #[cfg(debug_assertions)]
        println!("Close");
        #[cfg(not(debug_assertions))]
        _window
            .unwrap()
            .window()
            .dispatch_event(slint::platform::WindowEvent::CloseRequested);
    }
    pub fn new(settings: ArgOpts) -> Self {
        let tone = settings.tone.unwrap_or_default();

        Self {
            tone: Rc::new(RefCell::new(tone)),
            settings,
            ..Default::default()
        }
    }

    pub fn set_globals(&self) {
        let global = self.window.global::<MainState>();
        global.set_show_preview(self.settings.show_preview);
        global.set_show_search(self.settings.show_search);
        if let Some(font) = self.settings.font.as_deref() {
            global.set_font(SharedString::from(font));
        }

        let colors = self.window.global::<MyColors>();
        if let Some(color) = self.settings.background_color.as_ref() {
            colors.set_background(color.to_rgba().unwrap());
        }
        if let Some(color) = self.settings.primary_color.as_ref() {
            colors.set_foreground(color.to_rgba().unwrap());
        }
        self.window.set_emojis(self.content.clone());
    }

    pub fn set_events(&self) {
        self.window.on_start({
            let window = self.window.as_weak();
            move |width, height| {
                println!("Start invoked");
                let window = window.upgrade().unwrap();

                let size = (width as i32, height as i32);
                println!("{size:?}");

                let device_state = device_query::DeviceState::new();
                let pos = device_state.query_pointer().coords;
                let (x, y) = mouse_to_window_pos(size, pos);
                window.window().set_position(slint::WindowPosition::Logical(
                    slint::LogicalPosition::new(x as f32, y as f32),
                ));
            }
        });

        let global = self.window.global::<MainState>();
        global.set_enable_dbg(self.settings.debug);
        global.on_close({
            let window = self.window.as_weak();
            move || {
                Self::close(window.clone());
            }
        });
        self.window.global::<Navigation>().on_move({
            let window = self.window.as_weak();
            move |e| {
                window.upgrade().unwrap().window().move_focus(e);
            }
        });
    }

    pub fn run(&self) -> Result<(), slint::PlatformError> {
        let tabs = self.window.global::<TabsHandle>();
        tabs.set_tab(emojis::Group::SmileysAndEmotion as i32);
        tabs.set_tabs(get_default_tabs((*self.tone.borrow()).into()));

        tabs.on_change_tab({
            let content = self.content.clone();
            let tone = self.tone.clone();
            move |id| {
                let group = group_from(id);
                let tone = tone.borrow();
                let tone: emojis::SkinTone = (*tone).into();

                let emojis = group
                    .emojis()
                    .collect::<Vec<_>>()
                    .chunks(EMOJI_COLS)
                    .map(|e| {
                        ModelRc::from(
                            e.iter()
                                .cloned()
                                .flat_map(|e| e.with_skin_tone(tone).or(Some(e)))
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

        let search = self.window.global::<SearchGlobal>();
        search.set_tone(self.tone.clone().take());
        search.on_search({
            let tone = self.tone.clone();
            let window = self.window.as_weak();
            let content = self.content.clone();
            move |s| {
                let s = s.trim().to_lowercase();
                let tone = tone.borrow();
                let content = content
                    .as_any()
                    .downcast_ref::<VecModel<ModelRc<EmojiModel>>>()
                    .unwrap();
                if s.is_empty() {
                    let tab = window.upgrade().unwrap().global::<TabsHandle>().get_tab();
                    let group = group_from(tab);

                    let emojis = group
                        .emojis()
                        .collect::<Vec<_>>()
                        .chunks(EMOJI_COLS)
                        .map(|e| {
                            ModelRc::from(
                                e.iter()
                                    .cloned()
                                    .flat_map(|e| e.with_skin_tone((*tone).into()).or(Some(e)))
                                    .map(emoji_to_model)
                                    .collect::<Vec<_>>()
                                    .as_slice(),
                            )
                        })
                        .collect::<Vec<ModelRc<_>>>();
                    content.set_vec(emojis);
                    return;
                }
                let emojis = emojis::iter()
                    .filter(|&e| {
                        e.name().to_lowercase().contains(s.as_str())
                            || e.shortcodes()
                                .any(|c| c.to_lowercase().contains(s.as_str()))
                    })
                    .map(|e| emoji_to_model(e.with_skin_tone((*tone).into()).unwrap_or(e)))
                    .collect::<Vec<_>>()
                    .chunks(EMOJI_COLS)
                    .map(|e| ModelRc::from(e.to_vec().as_slice()))
                    .collect::<Vec<ModelRc<_>>>();

                content.set_vec(emojis);
            }
        });
        search.on_change_tone({
            let tone = self.tone.clone();
            let content = self.content.clone();
            let window = self.window.as_weak();
            move |t| {
                let t = t.as_str().parse().unwrap();
                tone.replace(t);
                let t = t.into();
                window
                    .unwrap()
                    .global::<TabsHandle>()
                    .set_tabs(get_default_tabs(t));

                let content = content
                    .as_any()
                    .downcast_ref::<VecModel<ModelRc<EmojiModel>>>()
                    .unwrap();
                let new_content: Vec<ModelRc<EmojiModel>> = content.iter().collect();
                content.set_vec(
                    new_content
                        .into_iter()
                        .map(|e| {
                            let e = e
                                .iter()
                                .map(|e| {
                                    let e = emoji_from_model(e);
                                    emoji_to_model(e.with_skin_tone(t).unwrap_or(e))
                                })
                                .collect::<Vec<_>>();
                            ModelRc::from(e.as_slice())
                        })
                        .collect::<Vec<ModelRc<_>>>(),
                );
            }
        });

        self.window.window().on_winit_window_event({
            let no_close = self.settings.no_close;
            move |w, ev| match ev {
                WindowEvent::CursorLeft { .. } => (!no_close)
                    .then(|| w.hide().ok())
                    .flatten()
                    .map(|_| WinitWindowEventResult::PreventDefault)
                    .unwrap_or(WinitWindowEventResult::Propagate),
                _ => WinitWindowEventResult::Propagate,
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
                        .upgrade()
                        .unwrap()
                        .window()
                        .dispatch_event(slint::platform::WindowEvent::CloseRequested);
                }
            }
        });

        self.window.run()
    }
}
