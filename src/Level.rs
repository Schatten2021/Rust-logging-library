//! All the different logging levels.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};
use crate::LogLevel;
/// The minimum log level
pub const MIN: LogLevel = LogLevel::MIN;

/// Log everything. Should only be used for setting the level of the logger.
pub const ALL: LogLevel = LogLevel::MIN;
/// Debug message used for debugging only.
pub const DEBUG: LogLevel = 0;
/// Information that the user should see.
pub const INFO: LogLevel = 10;
/// Distinct for messages of success.
pub const SUCCESS: LogLevel = 20;
/// When you want to warn the user but nothing has gone wrong yet.
/// Probably a good level to have in production (if you don't let the user see the level).
pub const WARN: LogLevel = 30;
/// For when an error occurs.
pub const ERROR: LogLevel = 40;
/// Critical messages that should be logged.
pub const CRITICAL: LogLevel = 50;
/// When something goes horribly, horribly wrong.
pub const FATAL: LogLevel = 60;
/// Log nothing. Should only be used for setting the level of the logger, because the message would always be logged.
pub const NONE: LogLevel = LogLevel::MAX;
/// The maximum log level
pub const MAX: LogLevel = LogLevel::MAX;
pub(crate) static LOG_LEVELS: OnceLock<RwLock<HashMap<LogLevel, Box<str>>>> = OnceLock::new();
fn _get_log_levels<'a>() -> &'a RwLock<HashMap<LogLevel, Box<str>>> {
    LOG_LEVELS.get_or_init(|| RwLock::new(HashMap::new()))
}
pub fn add_level(level: LogLevel, name: String) {
    let mut lock = _get_log_levels().write().expect("Log levels are poisoned");
    lock.insert(level, name.into_boxed_str());
}
pub fn get_level(level: LogLevel) -> Option<String> {
    let lock = _get_log_levels().read().expect("Log levels are poisoned");
    lock.get(&level).map(|name| name.to_string())
}