use crate::logging::{self, LogLevel};

pub const MIN_LEVEL: i32 = -1;
pub const MAX_LEVEL: i32 = 24;

pub struct Levels {
    pub current: i32,
    pub target: i32,
}

impl Levels {
    fn is_valid(level: i32) -> bool {
        (MIN_LEVEL..=MAX_LEVEL).contains(&level)
    }

    pub fn new(current: i32, target: i32) -> Option<Self> {
        if !Self::is_valid(current) {
            logging::log(
                LogLevel::Error,
                format!(
                    "current level {} out of range [{}, {}]",
                    current, MIN_LEVEL, MAX_LEVEL
                ),
            );
            return None;
        }

        if !Self::is_valid(target) {
            logging::log(
                LogLevel::Error,
                format!(
                    "target level {} out of range [{}, {}]",
                    target, MIN_LEVEL, MAX_LEVEL
                ),
            );
            return None;
        }

        Some(Self { current, target })
    }
}
