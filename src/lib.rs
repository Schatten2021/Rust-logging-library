// mod logger_old;
mod logger;
mod macros;
#[allow(non_snake_case)]
pub mod Level;

use std::sync::{Arc, RwLock};

#[cfg(feature = "coloured_output")]
use ansi_term::Color;

pub type LogLevel = i32;


#[derive(Clone)]
/// A logger used for logging messages at different levels.
/// Loggers are in a hierarchical structure, so sections of loggers can be turned on and off. 
pub struct Logger {
    inner: Arc<RwLock<logger::Logger>>,
}
impl Logger {
    /// Create a new logger.
    /// 
    /// # Arguments 
    /// 
    /// * `name`: The name of the logger. 
    /// Sub-logger can be created with a dot, so that `logging::Logger::new("foo::bar");` is a sub-logger of `logging::Logger::new("foo");`
    /// 
    /// 
    /// returns: Logger 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// let logger = logging::Logger::new("foo.bar");
    /// ```
    pub fn new(name: impl ToString) -> Self {
        Logger {
            inner: logger::get_logger(name.to_string()),
        }
    }
    /// Log a message.
    /// 
    /// # Arguments 
    /// 
    /// * `msg`: The message to be logged.
    /// * `level`: The level at which to log the message.
    /// 
    /// returns: () 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.log("Hello World".to_string(), Level::INFO);
    /// ```
    pub fn log(&self, msg: String, level: LogLevel) {
        let locked = self.inner.read().expect("Logger is poisoned");
        locked.log(msg, level)
    }
    /// Debug a message or value. Equal to [log](Logger::log)(msg, [Level::DEBUG](Level::DEBUG)).
    /// 
    /// # Arguments 
    /// 
    /// * `msg`: The message to be logged.
    /// 
    /// returns: () 
    /// 
    /// # Examples 
    /// 
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.debug("Hello World".to_string());
    /// ```
    pub fn debug(&self, msg: String) {
        self.log(msg, Level::DEBUG)
    }
    /// Log an information. Equal to [log](Logger::log)(msg, [Level::INFO](Level::INFO)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The message to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.info("Hello World".to_string());
    /// ```
    pub fn info(&self, msg: String) {
        self.log(msg, Level::INFO)
    }
    /// Log a success. Equal to [log](Logger::log)(msg, [Level::SUCCESS](Level::SUCCESS)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The success to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.success("Hello World".to_string());
    /// ```
    pub fn success(&self, msg: String) {
        self.log(msg, Level::SUCCESS)
    }

    /// Log a warning. Equal to [log](Logger::log)(msg, [Level::WARN](Level::WARN)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The success to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.success("Hello World".to_string());
    /// ```
    pub fn warn(&self, msg: String) {
        self.log(msg, Level::WARN);
    }
    /// Log an error. Equal to [log](Logger::log)(msg, [Level::ERROR](Level::ERROR)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The error to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.error("Hello World".to_string());
    /// ```
    pub fn error(&self, msg: String) {
        self.log(msg, Level::ERROR)
    }

    /// Log a message when something goes critically wrong. Equal to [log](Logger::log)(msg, [Level::CRITICAL](Level::CRITICAL)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The critical message to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.critical("Hello World".to_string());
    /// ```
    pub fn critical(&self, msg: String) {
        self.log(msg, Level::CRITICAL)
    }
    /// Log a message when something goes fatally wrong. Equal to [log](Logger::log)(msg, [Level::FATAL](Level::FATAL)).
    ///
    /// # Arguments
    ///
    /// * `msg`: The message to be logged.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo");
    /// logger.fatal("Hello World".to_string());
    /// ```
    pub fn fatal(&self, msg: String) {
        self.log(msg, Level::FATAL)
    }
    /// Set the minimum Level the logger and all children log at.
    ///
    /// # Arguments
    ///
    /// * `new_level`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// let logger = logging::Logger::new("foo");
    /// logger.set_level(Level::ALL);
    /// // will be logged
    /// logger.debug("Hello World".to_string());
    ///
    /// logger.set_level(Level::INFO);
    /// // will not be logged
    /// logger.debug("Hello World".to_string());
    /// // will be logged
    /// logger.info("Hello World".to_string());
    /// ```
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// let parent = logging::Logger::new("foo");
    /// let child = logging::Logger::new("foo.bar");
    /// parent.set_level(Level::INFO);
    /// // will be logged
    /// child.info("Hello World".to_string());
    /// // will not be logged
    /// child.debug("Hello World".to_string());
    /// child.set_level(Level::DEBUG);
    /// // will be logged
    /// child.debug("Hello World".to_string());
    /// // will not be logged
    /// parent.debug("Hello World".to_string())
    /// ```
    pub fn set_level(&self, new_level: LogLevel) {
        let mut locked = self.inner.write().expect("Logger is poisoned");
        locked.set_level(new_level)
    }
    /// Add a handler to this logger and all children (similar to [set_level](Logger::set_level)).
    /// Handlers are used to actually log the messages, e.g. the [CONSOLE_HANDLER](CONSOLE_HANDLER) will log messages to the console.
    /// without any handlers, the messages will not be saved/printed/etc.
    ///
    /// # Arguments
    ///
    /// * `handler`: The handler to add to the logger and all children.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    ///
    /// logging::set_level(Level::ALL);
    /// let logger = logging::Logger::new("foo".to_string());
    ///
    /// // will do nothing
    /// logger.info("This won't print".to_string());
    ///
    /// logging::add_handler(&CONSOLE_HANDLER);
    ///
    /// // now it will print to the console
    /// logger.info("This will print to the console. Maybe even in a coloured output (if you have that feature enabled).".to_string())
    /// ```
    pub fn add_handler<T: Handler + 'static>(&self, handler: T) {
        let mut locked = self.inner.write().expect("Logger is poisoned");
        locked.add_handler(Arc::new(handler))
    }
}
/// A handler for loggers.
/// These handle the messages and are responsible for logging the messages to whatever medium they are made to log to.
pub trait Handler: Send + Sync {
    /// Handle a message.
    /// This will log the message.
    ///
    /// # Arguments
    ///
    /// * `level`: The level the message is being logged at. Can be used for formating.
    /// * `message`: The actual String of the message. Should definitely be logged.
    /// * `logger`: The name of the logger doing the request to log the message. Can be formated in.
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    /// use logging::{Logger, Level, Handler};
    ///
    /// struct ConsoleHandler {}
    /// impl Handler for ConsoleHandler {
    ///     fn log(&self, level: Level, message: String, logger: String) {
    ///         println!("{} {:?}: {}", logger, level, message);
    ///     }
    /// }
    /// let logger = Logger::new("foo".to_string());
    /// logger.set_level(Level::ALL);
    ///
    /// // does nothing
    /// logger.info("won't log".to_string());
    ///
    /// logger.add_handler(&ConsoleHandler{});
    /// // will log
    /// logger.info("will print to console".to_string());
    ///
    /// ```
    fn log(&self, level: LogLevel, message: String, logger: String);
}
/// A default implementation of [Handler](Handler).
/// Logs to the console in a potentially coloured output (if you have the coloured_output feature enabled).
pub struct ConsoleHandler;
impl Handler for ConsoleHandler {
    fn log(&self, level: LogLevel, message: String, logger_name: String) {
        let level_name = Level::get_level(level).unwrap_or(level.to_string());
        let log_str = format!("{} ({}): {}", level_name, logger_name, message);
        #[cfg(feature = "coloured_output")]
        let log_str = {
            match level {
                Level::DEBUG => Color::Blue.normal(),
                Level::INFO => Color::Yellow.normal(),
                Level::SUCCESS => Color::Green.normal(),
                Level::WARN => Color::Red.italic(),
                Level::ERROR => Color::Red.normal(),
                Level::CRITICAL => Color::Red.bold(),
                Level::FATAL => Color::Red.bold().underline(),
                _ => Color::White.normal(),
            }.paint(log_str)
        };
        #[cfg(feature = "std_err")]
        if level >= Level::ERROR {
            eprintln!("{}", log_str);
        }
        println!("{}", log_str);
    }
}

/// Set the level globally to all loggers.
/// 
/// # Arguments 
/// 
/// * `level`: The new minimum level for all loggers. 
/// 
/// returns: () 
/// 
/// # Examples 
/// 
/// ```
/// use logging::{Level, Logger, CONSOLE_HANDLER};
/// let logger = Logger::new("foo");
/// logger.add_handler(&CONSOLE_HANDLER);
/// logger.set_level(Level::CRITICAL);
/// // won't log
/// logger.info("This won't log".to_string());
///
/// logging::set_level(Level::ALL);
/// // will log.
/// logger.info("This will log".to_string());
/// ```
pub fn set_level(level: LogLevel) {
    logger::get_root().write().expect("Logger poisoned").set_level(level)
}
/// Globally add a handler to all loggers.
/// 
/// # Arguments 
/// 
/// * `handler`: The new handler to be added.
/// 
/// returns: () 
/// 
/// # Examples 
/// 
/// ```
/// use logging::{CONSOLE_HANDLER, Logger, Level};
/// use logging::Level::CRITICAL;
/// logging::set_level(Level::ALL);
/// let logger = Logger::new("foo");
/// let logger2 = Logger::new("bar");
/// // only adds for 'logger'
/// logger.add_handler(&CONSOLE_HANDLER);
/// logger.debug("Will log.".to_string());
/// logger2.debug("Won't log.".to_string());
///
/// // adds it to all
/// logging::add_handler(&CONSOLE_HANDLER);
/// logger.debug("Will log twice, as the handler was added twice.".to_string());
/// logger2.debug("Will now also log.".to_string());
/// ```
pub fn add_handler<T: Handler + 'static>(handler: T) {
    logger::get_root().write().expect("Logger poisoned").add_handler(Arc::new(handler));
}