use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};
use crate::{Handler, Level};

static ROOT: OnceLock<Mutex<Logger>> = OnceLock::new();

pub(crate) struct Logger {
    level: Level,    handlers: Vec<Arc<Mutex<&'static dyn Handler>>>,
    name: String,
    children: HashMap<String, Arc<Mutex<Self>>>,
}
impl Logger {
    pub(crate) fn new(name: impl ToString) -> Arc<Mutex<Self>> {
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
            handler.lock().unwrap().log(level.clone(), message.clone(), self.name.clone())
        }
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
    fn get_child(&mut self, name: String) -> Arc<Mutex<Self>> {
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
    fn get_root() -> &'static Mutex<Self> {
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
pub fn set_level(level: Level) {
    Logger::get_root().lock().unwrap().set_level(level)
}
pub fn add_handler(handler: &'static impl Handler) {
    Logger::get_root().lock().unwrap().add_handler(handler);
}