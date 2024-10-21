use std::any::Any;
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


pub struct Logger {
    level: Level,
    handlers: Vec<Arc<Mutex<&'static dyn Handler>>>,
    name: String,
    children: HashMap<String, Arc<Mutex<Logger>>>,
}
impl Logger {
    pub fn new(name: impl ToString) -> Arc<Mutex<Logger>> {
        let root = Logger::get_root();
        let mut root_lock = root.lock().unwrap();
        let child = root_lock.get_child(name.to_string());
        child
    }
    pub fn log(&self, msg: impl ToString, level: Level) {
        if level < self.level {
            return;
        }
        let message = msg.to_string();
        for handler in &self.handlers {
            handler.lock().unwrap().log(level.clone(), message.clone(), self)
        }
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
        self.level = new_level.clone();
        for child in self.children.values_mut() {
            child.lock().unwrap().set_level(new_level.clone())
        }
    }
    pub fn add_handler(&mut self, handler: &'static impl Handler) {
        self.handlers.push(Arc::new(Mutex::new(handler)));
        for child in self.children.values_mut() {
            child.lock().unwrap().add_handler(handler)
        }
    }
}
impl Logger {
    fn get_child(&mut self, name: String) -> Arc<Mutex<Logger>> {
        let parts: Vec<&str> = name.splitn(2, ".").collect();
        let sub_logger_name = parts[0];
        if !self.children.contains_key(sub_logger_name) {
            let sub_logger = Logger {
                level: self.level.clone(),
                handlers: self.handlers.clone(),
                name: self.name.clone() + "." + sub_logger_name,
                children: HashMap::new(),
            };
            self.children.insert(sub_logger_name.to_string(), Arc::new(Mutex::new(sub_logger)));
        }
        let next_logger = self.children.get(sub_logger_name).expect("Should have inserted new logger.");
        if parts.len() == 1 {
            next_logger.clone()
        } else {
            let mut next_logger = next_logger.lock().unwrap();
            next_logger.get_child(parts[1].to_string())
        }
    }
    fn get_root() -> &'static Mutex<Logger> {
        ROOT.get_or_init(|| {
            Mutex::new(Logger {
                level: Level::NONE,
                handlers: Vec::new(),
                name: String::from("root"),
                children: HashMap::new(),
            })
        })
    }
}
pub trait Handler: Send + Sync {
    fn log(&self, level: Level, message: String, logger: &Logger);
}
pub const CONSOLE_HANDLER: ConsoleHandler = ConsoleHandler {  };

pub struct ConsoleHandler {}
impl ConsoleHandler {
    pub fn new() -> Self { ConsoleHandler {} }
}
impl Handler for ConsoleHandler {
    fn log(&self, level: Level, message: String, logger: &Logger) {
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
            println!("{}", col.paint(format!("{:?} ({}): {}", level, logger.name, message)))
        }
        #[cfg(not(feature = "coloured_output"))]
        println!("{:?} ({}): {}", level, logger.name, message)
    }
}

pub fn set_level(level: Level) {
    Logger::get_root().lock().unwrap().set_level(level)
}
pub fn add_handler(handler: &'static impl Handler) {
    Logger::get_root().lock().unwrap().add_handler(handler);
}