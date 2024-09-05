use std::fmt;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// Accept both String and &str so we can call this with string literals and format!().
pub fn log<T: Into<String> + fmt::Display>(level: LogLevel, msg: T) {
    let prefix = match level {
        LogLevel::Info => "info",
        LogLevel::Warning => "warning",
        LogLevel::Error => "error",
    };

    eprintln!("{}: {}", prefix, msg);
}
