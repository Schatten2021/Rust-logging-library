use crate::{Handler, LogLevel, CONSOLE_HANDLER};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

static ROOT: OnceLock<RwLock<Logger>> = OnceLock::new();


pub(crate) struct Logger {
    level: LogLevel,
    handlers: Vec<Arc<dyn Handler>>,
    name: Box<str>,
    children: HashMap<String, Arc<RwLock<Logger>>>,
}
impl Logger {
    pub(crate) fn log(&self, msg: String, level: LogLevel) -> () {
        if level < self.level {
            return;
        }
        for handler in &self.handlers {
            handler.log(level, msg.clone(), self.name.to_string());
        }
    }
    pub(crate) fn set_level(&mut self, level: LogLevel) {
        self.level = level;
        for child in self.children.values_mut() {
            let mut lock = child.write().expect("Logger is poisoned");
            lock.set_level(level);
        }
    }
    pub(crate) fn add_handler(&mut self, handler: Arc<dyn Handler>) {
        self.handlers.push(handler.clone());
        for child in self.children.values_mut() {
            let mut lock = child.write().expect("Logger is poisoned");
            lock.add_handler(handler.clone());
        }
    }
    fn get_child(&mut self, name: String) -> Arc<RwLock<Self>> {
        let remaining = &name[self.name.len()..];
        assert!(remaining.starts_with("::"), "invalid internal name. Logger passed to the wrong sublogger");
        let sub_name = remaining["::".len()..].split("::").next().expect("invalid name for logger");
        let sub_logger = match self.children.get(sub_name) {
            Some(sub_logger) => Arc::clone(sub_logger),
            None => {
                let logger = Arc::new(RwLock::new(Self {
                    level: self.level,
                    handlers: self.handlers.clone(),
                    name: format!("{}::{}", self.name, sub_name).into_boxed_str(),
                    children: HashMap::new(),
                }));
                self.children.insert(sub_name.to_string(), Arc::clone(&logger));
                logger
            }
        };
        if sub_name.len() + "::".len() == remaining.len() {
            // this is the final logger
            return sub_logger;
        }
        let mut lock = sub_logger.write().expect("Logger is poisoned");
        lock.get_child(name)
    }
}
pub(crate) fn get_logger(name: String) -> Arc<RwLock<Logger>> {
    get_root().write().expect("Logger is poisoned")
        .get_child(name)
}
pub(crate) fn get_root<'a>() -> &'a RwLock<Logger> {
    ROOT.get_or_init(|| {
        RwLock::new(Logger {
            level: LogLevel::MAX,
            #[cfg(not(feature = "default_log_console"))]
            handlers: vec![],
            #[cfg(feature = "default_log_console")]
            handlers: vec![Arc::new(CONSOLE_HANDLER)],
            name: Box::from(""),
            children: HashMap::new(),
        })
    })
}