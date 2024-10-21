mod logger;

#[cfg(feature = "coloured_output")]
use ansi_term::Color;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

static ROOT: OnceLock<Mutex<Logger>> = OnceLock::new();


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Level {
    ALL = 0,
    DEBUG = 1,
    INFO = 2,
    SUCCESS = 3,
    ERROR = 4,
    CRITICAL = 5,
    FATAL = 6,
    NONE = 7,
}

#[derive(Clone)]
pub struct Logger {
    inner: Arc<Mutex<logger::Logger>>,
}
impl Logger {
    pub fn new(name: impl ToString) -> Self {
        Logger {
            inner: logger::Logger::new(name)
        }
    }
    pub fn log(&self, msg: impl ToString, level: Level) {
        let locked = self.inner.lock().unwrap();
        locked.log(msg, level)
    }
    pub fn debug(&self, msg: impl ToString) {
        self.log(msg, Level::DEBUG)
    }
    pub fn info(&self, msg: impl ToString) {
        self.log(msg, Level::INFO)
    }
    pub fn success(&self, msg: impl ToString) {
        self.log(msg, Level::SUCCESS)
    }
    pub fn error(&self, msg: impl ToString) {
        self.log(msg, Level::ERROR)
    }
    pub fn critical(&self, msg: impl ToString) {
        self.log(msg, Level::CRITICAL)
    }
    pub fn fatal(&self, msg: impl ToString) {
        self.log(msg, Level::FATAL)
    }
    pub fn set_level(&mut self, new_level: Level) {
        let mut locked = self.inner.lock().unwrap();
        locked.set_level(new_level)
    }
    pub fn add_handler(&mut self, handler: &'static impl Handler) {
        let mut locked = self.inner.lock().unwrap();
        locked.add_handler(handler)
    }
}
pub trait Handler: Send + Sync {
    fn log(&self, level: Level, message: String, logger: String);
}
pub const CONSOLE_HANDLER: ConsoleHandler = ConsoleHandler {  };

pub struct ConsoleHandler {}
impl ConsoleHandler {
    pub fn new() -> Self { ConsoleHandler {} }
}
impl Handler for ConsoleHandler {
    fn log(&self, level: Level, message: String, logger_name: String) {
        #[cfg(feature = "coloured_output")]
        {
            let col = match level {
                Level::ALL => Color::White.normal(),
                Level::DEBUG => Color::Blue.normal(),
                Level::INFO => Color::Yellow.normal(),
                Level::SUCCESS => Color::Green.normal(),
                Level::ERROR => Color::Red.normal(),
                Level::CRITICAL => Color::Red.italic(),
                Level::FATAL => Color::Red.bold().underline(),
                Level::NONE => { return; },
            };
            println!("{}", col.paint(format!("{:?} ({}): {}", level, logger_name, message)))
        }
        #[cfg(not(feature = "coloured_output"))]
        println!("{:?} ({}): {}", level, logger_name, message)
    }
}

pub fn set_level(level: Level) {
    logger::set_level(level)
}
pub fn add_handler(handler: &'static impl Handler) {
    logger::add_handler(handler);
}