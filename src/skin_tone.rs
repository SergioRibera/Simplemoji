use std::str::FromStr;

use crate::SkinTone;

impl From<SkinTone> for emojis::SkinTone {
    fn from(value: SkinTone) -> Self {
        match value {
            SkinTone::Default => emojis::SkinTone::Default,
            SkinTone::Light => emojis::SkinTone::Light,
            SkinTone::MediumLight => emojis::SkinTone::MediumLight,
            SkinTone::Medium => emojis::SkinTone::Medium,
            SkinTone::MediumDark => emojis::SkinTone::MediumDark,
            SkinTone::Dark => emojis::SkinTone::Dark,
        }
    }
}

impl FromStr for SkinTone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "👋🏻" => Ok(SkinTone::Light),
            "👋🏼" => Ok(SkinTone::MediumLight),
            "👋🏽" => Ok(SkinTone::Medium),
            "👋🏾" => Ok(SkinTone::MediumDark),
            "👋🏿" => Ok(SkinTone::Dark),
            _ => Ok(SkinTone::Default),
        }
    }
}
