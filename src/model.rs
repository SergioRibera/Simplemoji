use std::fmt::Display;

use clap::ValueEnum;

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SkinTone {
    #[default]
    Default,
    Light,
    MediumLight,
    Medium,
    MediumDark,
    Dark,
}

impl SkinTone {
    pub fn get_emoji(&self) -> String {
        emojis::Group::PeopleAndBody
            .emojis()
            .next()
            .unwrap()
            .with_skin_tone((*self).clone().into())
            .unwrap()
            .to_string()
    }

    pub fn get_all() -> &'static [SkinTone] {
        &[
            SkinTone::Default,
            SkinTone::Light,
            SkinTone::MediumLight,
            SkinTone::Medium,
            SkinTone::MediumDark,
            SkinTone::Dark,
        ]
    }
}

impl Into<emojis::SkinTone> for SkinTone {
    fn into(self) -> emojis::SkinTone {
        match self {
            SkinTone::Default => emojis::SkinTone::Default,
            SkinTone::Light => emojis::SkinTone::Light,
            SkinTone::MediumLight => emojis::SkinTone::MediumLight,
            SkinTone::Medium => emojis::SkinTone::Medium,
            SkinTone::MediumDark => emojis::SkinTone::MediumDark,
            SkinTone::Dark => emojis::SkinTone::Dark,
        }
    }
}

impl Display for SkinTone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.get_emoji().as_str())
    }
}
