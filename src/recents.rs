use ui::emojis;
use ui::{EmojiModel, SkinTone};

use crate::EMOJI_COLS;
use crate::app_data::app_data_dir;
use crate::utils::emoji_to_model;
use std::cell::RefMut;
use std::{
    fs::OpenOptions,
    io::{Error, Read},
};

#[derive(clap::ValueEnum, Clone, Debug, Default)]
pub enum RecentType {
    #[default]
    MostUsed,
    Mixed,
    PopPush,
}

pub fn load_recent(tone: SkinTone) -> Result<Vec<EmojiModel>, Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .open(app_data_dir().join("recent_emojis.semj"))?;

    #[cfg(target_endian = "big")]
    let config = bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    #[cfg(target_endian = "little")]
    let config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();

    let mut bytes: Vec<u8> = vec![];
    file.read_to_end(&mut bytes)?;
    let recent =
        bincode::decode_from_slice::<Vec<String>, _>(&bytes, config).unwrap_or((vec![], 0));

    let recent: Vec<EmojiModel> = recent
        .0
        .into_iter()
        .filter_map(|e| emojis::get(&e))
        .filter_map(|e| e.with_skin_tone(tone.into()).or(Some(e)))
        .map(emoji_to_model)
        .collect();

    Ok(recent)
}

pub fn most_used(mut recents: RefMut<Vec<Vec<EmojiModel>>>, emoji: EmojiModel, rows: u8) {
    let recents_new = recents.clone();
    let mut recents_new: Vec<EmojiModel> = recents_new.into_iter().flatten().collect();
    let emoji_idx = recents_new.iter().position(|e| e.name == emoji.name);

    if recents_new.len() < EMOJI_COLS * rows as usize && emoji_idx.is_none() {
        recents_new.push(emoji.clone());
    }

    if let Some(idx) = emoji_idx
        && idx > 0
    {
        recents_new.swap(idx - 1, idx);
    } else if let Some(idx) = emoji_idx
        && idx == 0
    {
        return;
    } else {
        let Some(last) = recents_new.last_mut() else {
            return;
        };
        *last = emoji;
    }

    let recents_new: Vec<Vec<EmojiModel>> = recents_new
        .chunks(EMOJI_COLS)
        .map(<[ui::EmojiModel]>::to_vec)
        .collect();
    *recents = recents_new;
}

pub fn pop_push(mut recents: RefMut<Vec<Vec<EmojiModel>>>, emoji: EmojiModel, rows: u8) {
    let recents_new = recents.clone();
    let mut recents_new: Vec<EmojiModel> = recents_new.into_iter().flatten().collect();
    let emoji_idx = recents_new.iter().position(|e| e.name == emoji.name);

    if recents_new.len() < EMOJI_COLS * rows as usize && emoji_idx.is_none() {
        recents_new.push(emoji.clone());
    } else if recents_new.len() == EMOJI_COLS * rows as usize && emoji_idx.is_none() {
        recents_new.remove(0);
        recents_new.push(emoji);
    }

    let recents_new: Vec<Vec<EmojiModel>> = recents_new
        .chunks(EMOJI_COLS)
        .map(<[ui::EmojiModel]>::to_vec)
        .collect();
    *recents = recents_new;
}

pub fn mixed(mut recents: RefMut<Vec<Vec<EmojiModel>>>, emoji: EmojiModel, rows: u8, n: usize) {
    let recents_new = recents.clone();
    let mut recents_new: Vec<EmojiModel> = recents_new.into_iter().flatten().collect();
    let emoji_idx = recents_new.iter().position(|e| e.name == emoji.name);

    if recents_new.len() < EMOJI_COLS * rows as usize && emoji_idx.is_none() {
        recents_new.push(emoji.clone());
    } else if recents_new.len() == EMOJI_COLS * rows as usize && emoji_idx.is_none() {
        recents_new.remove(n);
        recents_new.push(emoji);
    } else if let Some(idx) = emoji_idx
        && idx > n
    {
        recents_new.swap(n - 1, idx);
    } else if let Some(idx) = emoji_idx
        && idx <= n
        && idx > 0
    {
        recents_new.swap(idx, idx - 1);
    }

    let recents_new: Vec<Vec<EmojiModel>> = recents_new
        .chunks(EMOJI_COLS)
        .map(<[ui::EmojiModel]>::to_vec)
        .collect();
    *recents = recents_new;
}

pub fn save_recent(recents: &[Vec<EmojiModel>]) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .append(false)
        .write(true)
        .open(app_data_dir().join("recent_emojis.semj"))?;

    #[cfg(target_endian = "big")]
    let config = bincode::config::standard()
        .with_big_endian()
        .with_fixed_int_encoding();

    #[cfg(target_endian = "little")]
    let config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();

    let recents: Vec<String> = recents
        .iter()
        .flatten()
        .map(|e| e.character.to_string())
        .collect();

    _ = bincode::encode_into_std_write(recents.clone(), &mut file, config).unwrap();

    Ok(())
}
