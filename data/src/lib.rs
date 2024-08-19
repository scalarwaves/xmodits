//! Data components of XMODITS

pub mod config;
pub mod theme;
pub mod time;

pub use config::Config;
pub use theme::Theme;
pub use time::Time;

use xmodits_lib::export::Format;

pub const SUPPORTED_FORMATS: &[Format] = &[
    Format::WAV,
    Format::AIFF,
    Format::ITS,
    Format::S3I,
    Format::IFF,
    Format::RAW,
];

#[cfg(feature = "manual")]
pub static MANUAL: &str = include_str!("../../assets/manual.txt");