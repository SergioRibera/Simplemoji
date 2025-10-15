use std::sync::{Arc, Mutex};

use arboard::Clipboard;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use slint::winit_030::winit::event::WindowEvent;
use slint::winit_030::{EventResult, WinitWindowAccessor};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::color::ToRgba;
use crate::navigation::MoveFocus;
use crate::settings::ArgOpts;
use crate::utils::{
    emoji_from_model, emoji_to_model, emojis_from_group, get_default_tabs, group_from,
};
use crate::EMOJI_COLS;
use ui::{
    emojis, EmojiHandle, EmojiModel, MainState, MainWindow, MyColors, Navigation, SearchGlobal,
    SkinTone, TabsHandle,
};

pub struct MainApp {
    window: MainWindow,
    settings: ArgOpts,
    tone: Arc<Mutex<SkinTone>>,

    content: ModelRc<ModelRc<EmojiModel>>,
}

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
            tone: Arc::new(Mutex::new(tone)),
            settings,
            window: MainWindow::new().unwrap(),
            content: emojis_from_group(emojis::Group::SmileysAndEmotion),
        }
    }

    pub fn window(&self) -> Weak<MainWindow> {
        self.window.as_weak()
    }

    pub fn set_globals(&self) {
        let global = self.window.global::<MainState>();
        global.set_show_preview(self.settings.show_preview);
        global.set_show_search(self.settings.show_search);
        if let Some(corner_radius) = self.settings.corner_radius {
            global.set_corner_radius(corner_radius as _);
        }
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
        let tone = { *self.tone.lock().unwrap() };
        tabs.set_tab(emojis::Group::SmileysAndEmotion as i32);
        tabs.set_tabs(get_default_tabs(tone.into()));

        tabs.on_change_tab({
            let content = self.content.clone();
            let tone = self.tone.clone();
            move |id| {
                let group = group_from(id);
                let tone = { tone.lock().unwrap() };
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
        search.set_tone(tone);
        search.on_search({
            let tone = self.tone.clone();
            let window = self.window.as_weak();
            let content = self.content.clone();

            let use_fuzze = self.settings.fuzzing_search;
            let matcher = SkimMatcherV2::default().smart_case().use_cache(true);
            let matches_search = move |i: usize, s: String, e: &'static emojis::Emoji| -> bool {
                if !use_fuzze {
                    return e.name().to_lowercase().contains(&s)
                        || e.shortcodes().any(|c| c.to_lowercase().contains(&s));
                }

                let a = matcher.fuzzy_match(e.name(), &s).unwrap_or_default();
                let shortcodes = e.shortcodes();
                let shortcodes_count = shortcodes.clone().count();
                let shortcodes_count = if shortcodes_count == 0 {
                    1
                } else {
                    shortcodes_count
                };
                let b = shortcodes
                    .map(|e| matcher.fuzzy_match(e, &s).unwrap_or_default())
                    .sum::<i64>()
                    .saturating_div(shortcodes_count as i64);
                let c = (a.saturating_mul(4))
                    .saturating_add(b.saturating_mul(1))
                    .saturating_sub(i as i64);

                c > 0
            };
            move |s| {
                let s = s.trim().to_lowercase();
                let tone = { tone.lock().unwrap() };
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
                    .enumerate()
                    .flat_map(|(i, e)| {
                        (matches_search(i, s.clone(), e)).then_some(emoji_to_model(
                            e.with_skin_tone((*tone).into()).unwrap_or(e),
                        ))
                    })
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
                {
                    *tone.lock().unwrap() = t;
                }
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
            move |_w, ev| match ev {
                WindowEvent::CursorLeft { .. } => (!no_close)
                    .then(|| slint::quit_event_loop().ok())
                    .flatten()
                    .map(|_| EventResult::PreventDefault)
                    .unwrap_or(EventResult::Propagate),
                _ => EventResult::Propagate,
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
