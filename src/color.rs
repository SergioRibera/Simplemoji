use std::error::Error;
use std::num::ParseIntError;

use iced::Color;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseColor {
    InvalidLength,
    InvalidDigit,
    Parse(ParseIntError),
}

pub trait ToRgba {
    type Target;
    fn to_rgba(&self) -> Self::Target;
}

/// Parse hex color (#RRGGBB or #RRGGBBAA)
impl ToRgba for String {
    type Target = Result<Color, ParseColor>;

    fn to_rgba(&self) -> Self::Target {
        log::trace!("Start Parse Color: '{self}'");
        if self.is_empty() {
            return Err(ParseColor::InvalidLength);
        }
        if self.as_bytes()[0] != b'#' {
            return Err(ParseColor::InvalidDigit);
        }
        let mut color = u32::from_str_radix(&self[1..], 16)?;

        match self.len() {
            // RGB or RGBA
            4 | 5 => {
                let a = if self.len() == 5 {
                    log::trace!("Parse Alpha Color for RGBA");
                    let alpha = (color & 0xf) as u8;
                    color >>= 4;
                    alpha
                } else {
                    log::trace!("Default Alpha Color RGB");
                    0xff
                };

                let r = ((color >> 8) & 0xf) as u8;
                let g = ((color >> 4) & 0xf) as u8;
                let b = (color & 0xf) as u8;

                Ok(Color::from_rgba8(
                    (r << 4) | r,
                    (g << 4) | g,
                    (b << 4) | b,
                    (((a << 4) | a) as f32) / 255.,
                ))
            }
            // RRGGBB or RRGGBBAA
            7 | 9 => {
                let alpha = if self.len() == 9 {
                    log::trace!("Parse Alpha Color RRGGBBAA");
                    let alpha = (color & 0xff) as u8;
                    color >>= 8;
                    alpha
                } else {
                    log::trace!("Default Alpha Color RRGGBB");
                    0xff
                };

                Ok(Color::from_rgba8(
                    (color >> 16) as u8,
                    (color >> 8) as u8,
                    color as u8,
                    (alpha as f32) / 255.,
                ))
            }
            _ => Err(ParseColor::InvalidLength),
        }
    }
}

impl ToRgba for &str {
    type Target = Result<Color, ParseColor>;

    fn to_rgba(&self) -> Self::Target {
        String::from(*self).to_rgba()
    }
}

// Impl Errors
impl From<ParseIntError> for ParseColor {
    fn from(value: ParseIntError) -> Self {
        Self::Parse(value)
    }
}
impl std::fmt::Display for ParseColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseColor::InvalidLength => write!(f, "Invalid length of String"),
            ParseColor::InvalidDigit => write!(f, "Invalid digit"),
            ParseColor::Parse(v) => write!(f, "Error parsing number: {v}"),
        }
    }
}
impl Error for ParseColor {}
