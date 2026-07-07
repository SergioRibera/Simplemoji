use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use arboard::Clipboard;
use imekit::InputMethodEvent;

use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use slint::winit_030::winit::event::WindowEvent;
use slint::winit_030::{EventResult, WinitWindowAccessor};
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel, Weak};

use crate::EMOJI_COLS;
use crate::color::ToRgba;
use crate::navigation::MoveFocus;
use crate::recents::{RecentType, load_recent, mixed, most_used, pop_push, save_recent};
use crate::settings::ArgOpts;
use crate::utils::{
    emoji_from_model, emoji_to_model, emojis_from_group, get_default_tabs, group_from,
};
use ui::{
    EmojiHandle, EmojiModel, MainState, MainWindow, MyColors, Navigation, SearchGlobal, SkinTone,
    TabsHandle, emojis,
};

pub struct MainApp {
    window: Option<MainWindow>,
    settings: ArgOpts,
    tone: Arc<Mutex<SkinTone>>,
    content: ModelRc<ModelRc<EmojiModel>>,
    recent: Rc<RefCell<Vec<Vec<EmojiModel>>>>,
    pending_emoji: Rc<RefCell<Option<String>>>,
}

impl MainApp {
    fn win(&self) -> &MainWindow {
        self.window.as_ref().expect("window used after drop")
    }

    fn close(_window: Weak<MainWindow>, recent: Rc<RefCell<Vec<Vec<EmojiModel>>>>) {
        save_recent(&recent.borrow().to_vec()).unwrap();
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

        let mut recents: Vec<Vec<EmojiModel>> = load_recent(tone)
            .unwrap()
            .chunks(EMOJI_COLS)
            .map(<[ui::EmojiModel]>::to_vec)
            .collect();

        let recent_rows = settings.recent_rows;

        let recents = if recents.is_empty() {
            vec![Vec::<EmojiModel>::new(); recent_rows as usize]
        } else if recents.len() < recent_rows as usize {
            let extension: Vec<Vec<EmojiModel>> =
                vec![vec![]; recent_rows as usize - recents.len()];
            recents.extend_from_slice(&extension);
            recents
        } else {
            recents
        };

        let content = emojis_from_group(emojis::Group::SmileysAndEmotion);

        let mut model = recents.clone();
        model.extend(content);

        Self {
            tone: Arc::new(Mutex::new(tone)),
            settings,
            window: Some(MainWindow::new().unwrap()),
            recent: Rc::new(RefCell::from(recents)),
            content: vec_to_model(&model),
            pending_emoji: Rc::new(RefCell::new(None)),
        }
    }

    pub fn window(&self) -> Weak<MainWindow> {
        self.win().as_weak()
    }

    pub fn set_globals(&self) {
        let global = self.win().global::<MainState>();
        global.set_show_preview(self.settings.show_preview);
        global.set_show_search(self.settings.show_search);
        global.set_show_recent(self.settings.show_recent);

        global.set_recent_rows(self.settings.recent_rows.into());

        if let Some(corner_radius) = self.settings.corner_radius {
            global.set_corner_radius(corner_radius.into());
        }

        if let Some(font) = self.settings.font.as_deref() {
            global.set_font(SharedString::from(font));
        }

        let colors = self.win().global::<MyColors>();
        if let Some(color) = self.settings.background_color.as_ref() {
            colors.set_background(color.to_rgba().unwrap());
        }
        if let Some(color) = self.settings.primary_color.as_ref() {
            colors.set_foreground(color.to_rgba().unwrap());
        }
        self.win().set_emojis(self.content.clone());
    }

    pub fn set_events(&self) {
        let global = self.win().global::<MainState>();
        global.set_enable_dbg(self.settings.debug);
        global.on_close({
            let window = self.win().as_weak();
            let recent = self.recent.clone();
            move || {
                Self::close(window.clone(), recent.clone());
            }
        });
        self.win().global::<Navigation>().on_move({
            let window = self.win().as_weak();
            move |e| {
                window.upgrade().unwrap().window().move_focus(e);
            }
        });
    }
    #[allow(clippy::too_many_lines)]
    pub fn run(&mut self) -> Result<(), slint::PlatformError> {
        let tabs = self.win().global::<TabsHandle>();
        let tone = { *self.tone.lock().unwrap() };

        tabs.set_tab(emojis::Group::SmileysAndEmotion as i32);
        tabs.set_tabs(get_default_tabs(tone.into()));

        tabs.on_change_tab({
            let content = self.content.clone();
            let tone = self.tone.clone();
            let recents = self.recent.clone();

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
                                .copied()
                                .filter_map(|e| e.with_skin_tone(tone).or(Some(e)))
                                .map(emoji_to_model)
                                .collect::<Vec<_>>()
                                .as_slice(),
                        )
                    })
                    .collect::<Vec<ModelRc<_>>>();

                let recents: Vec<ModelRc<EmojiModel>> = recents
                    .borrow()
                    .iter()
                    .map(|col| ModelRc::from(col.as_ref()))
                    .collect();

                content
                    .as_any()
                    .downcast_ref::<VecModel<ModelRc<EmojiModel>>>()
                    .unwrap()
                    .set_vec([recents, emojis].concat());
            }
        });

        let search = self.win().global::<SearchGlobal>();
        search.set_tone(tone);
        search.on_search({
            let tone = self.tone.clone();
            let window = self.win().as_weak();
            let content = self.content.clone();

            let use_fuzze = self.settings.fuzzing_search;
            let matcher = SkimMatcherV2::default();
            let matches_search = move |s: String, e: &'static emojis::Emoji| -> bool {
                if !use_fuzze {
                    return e.name().to_lowercase().contains(&s)
                        || e.shortcodes().any(|c| c.to_lowercase().contains(&s));
                }

                let name_score = matcher.fuzzy_match(e.name(), &s).unwrap_or_default();
                let shortcode_score = e
                    .shortcodes()
                    .filter_map(|shortcode| matcher.fuzzy_match(shortcode, &s))
                    .max()
                    .unwrap_or_default();

                name_score.max(shortcode_score) > 0
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
                                    .copied()
                                    .filter_map(|e| e.with_skin_tone((*tone).into()).or(Some(e)))
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
                    .filter_map(|e| {
                        (matches_search(s.clone(), e)).then_some(emoji_to_model(
                            e.with_skin_tone((*tone).into()).unwrap_or(e),
                        ))
                    })
                    .collect::<Vec<_>>()
                    .chunks(EMOJI_COLS)
                    .map(ModelRc::from)
                    .collect::<Vec<ModelRc<_>>>();

                content.set_vec(emojis);
            }
        });
        search.on_change_tone({
            let tone = self.tone.clone();
            let content = self.content.clone();
            let window = self.win().as_weak();
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
                                    let e = emoji_from_model(&e);
                                    emoji_to_model(e.with_skin_tone(t).unwrap_or(e))
                                })
                                .collect::<Vec<_>>();
                            ModelRc::from(e.as_slice())
                        })
                        .collect::<Vec<ModelRc<_>>>(),
                );
            }
        });

        self.win().window().on_winit_window_event({
            let no_close = self.settings.no_close;
            let recent = self.recent.clone();

            move |_w, ev| match ev {
                WindowEvent::CursorLeft { .. } => {
                    save_recent(&recent.borrow().to_vec()).unwrap();
                    (!no_close)
                        .then(|| slint::quit_event_loop().ok())
                        .flatten()
                        .map_or(EventResult::Propagate, |()| EventResult::PreventDefault)
                }
                _ => EventResult::Propagate,
            }
        });

        self.win().global::<EmojiHandle>().on_click({
            let window = self.win().as_weak();
            let close = self.settings.close_on_copy;
            let content = self.content.clone();
            let recent = self.recent.clone();
            let recent_type = self.settings.recent_type.clone().unwrap_or_default();
            let recent_rows = self.settings.recent_rows;
            let static_recents = self.settings.static_recents;
            let cmd = self.settings.copy_command.clone();
            // Use IME only when no copy-command is set and IME is available.
            // Commit happens after the window closes so the previous input has focus.
            let use_ime = cmd.is_none() && self.settings.ime.is_some();
            let pending_emoji = self.pending_emoji.clone();
            let mut clipboard = (!use_ime && cmd.is_none())
                .then(|| {
                    Clipboard::new()
                        .inspect_err(|e| log::error!("Fail to create clipboard: {e}"))
                        .ok()
                })
                .flatten();

            move |emoji| {
                if use_ime {
                    *pending_emoji.borrow_mut() = Some(emoji.to_string());
                } else if let Some(cmd) = cmd.as_deref() {
                    let mut parts = cmd.split(' ');
                    let bin = parts.next().unwrap();
                    let mut args = parts.collect::<Vec<&str>>();
                    args.push(&emoji);
                    _ = std::process::Command::new(bin)
                        .args(args)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();
                } else if let Some(clipboard) = clipboard.as_mut() {
                    clipboard
                        .set_text(emoji.as_str())
                        .inspect_err(|e| log::error!("Fail to copy emoji to clipboard: {e}"))
                        .ok();
                }

                match recent_type {
                    RecentType::MostUsed => most_used(
                        recent.borrow_mut(),
                        emoji_to_model(emojis::get(&emoji).unwrap()),
                        recent_rows,
                    ),
                    RecentType::Mixed => mixed(
                        recent.borrow_mut(),
                        emoji_to_model(emojis::get(&emoji).unwrap()),
                        recent_rows,
                        static_recents,
                    ),
                    RecentType::PopPush => pop_push(
                        recent.borrow_mut(),
                        emoji_to_model(emojis::get(&emoji).unwrap()),
                        recent_rows,
                    ),
                }

                let content = content
                    .as_any()
                    .downcast_ref::<VecModel<ModelRc<EmojiModel>>>()
                    .unwrap();

                for (i, row) in recent.borrow().iter().enumerate() {
                    content.set_row_data(i, ModelRc::from(row.as_ref()));
                }

                if use_ime || close {
                    save_recent(&recent.borrow().to_vec()).unwrap();
                    window
                        .upgrade()
                        .unwrap()
                        .window()
                        .dispatch_event(slint::platform::WindowEvent::CloseRequested);
                }
            }
        });

        // Take the window out so it's dropped when run() returns, destroying the
        // Wayland surface before commit_pending waits for the compositor's Activate.
        self.window.take().unwrap().run()
    }

    pub fn commit_pending(&mut self) {
        let emoji = match self.pending_emoji.borrow_mut().take() {
            Some(e) => e,
            None => {
                log::debug!("commit_pending: no pending emoji, skipping");
                return;
            }
        };

        log::info!("commit_pending: emoji={emoji}");

        if self.settings.ime.is_none() {
            log::info!("commit_pending: no IME available, using clipboard fallback");
            self.clipboard_fallback(&emoji);
            return;
        }

        let committed = self.settings.ime.as_mut().map_or(false, |ime| {
            #[cfg(target_os = "linux")]
            log::debug!(
                "commit_pending: backend is_x11={} is_wayland={} is_active={}",
                ime.is_x11(),
                ime.is_wayland(),
                ime.is_active(),
            );
            #[cfg(not(target_os = "linux"))]
            log::debug!("commit_pending: backend is_active={}", ime.is_active());

            #[cfg(target_os = "linux")]
            if ime.is_x11() {
                // X11/XTest injects fake key events to the focused window directly;
                // no activation protocol needed.
                log::info!("commit_pending: X11/XTest path, committing directly");
                return ime
                    .commit_string(&emoji)
                    .inspect_err(|e| log::error!("X11 XTest commit_string: {e}"))
                    .is_ok();
            }

            // Drain stale events that accumulated while the window was open.
            // The terminal may have re-gained focus (and triggered Activate) during
            // the window-close transition — in that case is_active() will be true
            // after drain and we can commit immediately without sleeping.
            let mut drained = 0usize;
            while let Some(ev) = ime.next_event() {
                log::trace!("commit_pending: stale[{drained}] {ev:?}");
                drained += 1;
            }
            log::debug!(
                "commit_pending: drained {drained} stale events, is_active={}",
                ime.is_active()
            );

            if ime.is_active() {
                // Terminal re-activated during window close — use the current serial.
                let serial = ime.state().serial;
                log::info!("commit_pending: IME already active (serial={serial}), committing");
                return ime
                    .commit_string(&emoji)
                    .inspect_err(|e| log::error!("IME commit_string: {e}"))
                    .and_then(|_| {
                        ime.commit(serial)
                            .inspect_err(|e| log::error!("IME commit: {e}"))
                    })
                    .is_ok();
            }

            // IME not yet active; sleep so the compositor finishes processing the
            // window destruction and re-activates the previously focused text input.
            log::debug!("commit_pending: sleeping 150ms for focus restore");
            std::thread::sleep(Duration::from_millis(150));

            log::info!("commit_pending: waiting for Activate event (Wayland/IBus)");
            let deadline = Instant::now() + Duration::from_secs(2);
            loop {
                if Instant::now() >= deadline {
                    log::warn!("commit_pending: Activate timeout after 2s");
                    return false;
                }
                match ime.next_event() {
                    Some(InputMethodEvent::Activate { serial }) => {
                        log::info!("commit_pending: got Activate serial={serial}, committing");
                        return ime
                            .commit_string(&emoji)
                            .inspect_err(|e| log::error!("IME commit_string: {e}"))
                            .and_then(|_| {
                                ime.commit(serial)
                                    .inspect_err(|e| log::error!("IME commit: {e}"))
                            })
                            .is_ok();
                    }
                    Some(ev) => {
                        log::debug!("commit_pending: ignoring event {ev:?}");
                    }
                    None => std::thread::sleep(Duration::from_millis(5)),
                }
            }
        });

        if !committed {
            log::info!("commit_pending: IME commit failed, falling back to clipboard");
            self.clipboard_fallback(&emoji);
        } else {
            log::info!("commit_pending: done");
        }
    }

    fn clipboard_fallback(&self, emoji: &str) {
        if let Some(cmd) = self.settings.copy_command.as_deref() {
            let mut parts = cmd.split(' ');
            let bin = parts.next().unwrap();
            let args: Vec<&str> = parts.collect();
            _ = std::process::Command::new(bin)
                .args(args)
                .arg(emoji)
                .spawn()
                .and_then(|mut c| c.wait());
        } else if let Ok(mut clipboard) = Clipboard::new() {
            clipboard
                .set_text(emoji)
                .inspect_err(|e| log::error!("Fail to copy emoji to clipboard: {e}"))
                .ok();
        }
    }
}

fn vec_to_model(vec: &[Vec<EmojiModel>]) -> ModelRc<ModelRc<EmojiModel>> {
    let vec: Vec<ModelRc<EmojiModel>> = vec.iter().map(|v| ModelRc::from(v.as_ref())).collect();

    ModelRc::from(vec.as_ref())
}
