use crate::color::{self, Color};
use std::fmt;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// Accept both String and &str so we can call this with string literals and format!().
#[rustfmt::skip]
pub fn log<T: Into<String> + fmt::Display>(level: LogLevel, msg: T) {
    match level {
        LogLevel::Info => eprintln!("{}", msg),
        LogLevel::Warning => {
            eprintln!("{}: {}", color::color_text("warning", Color::BrightYellow), msg);
        }
        LogLevel::Error => {
            eprintln!("{}: {}", color::color_text("error", Color::BrightRed), msg);
        }
    }
}
