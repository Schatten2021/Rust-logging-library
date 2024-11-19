mod logger;
use std::sync::{Arc, Mutex};

#[cfg(feature = "coloured_output")]
use ansi_term::Color;


#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
/// All the different logging levels.
pub enum Level {
    /// Log everything. Should only be used for setting the level of the logger.
    ALL = 0,
    /// Debug message used for debugging only.
    DEBUG = 1,
    /// Information that the user should see.
    INFO = 2,
    /// Distinct for messages of success.
    SUCCESS = 3,
    /// For when an error occurs.
    ERROR = 4,
    /// Critical messages that should be logged.
    CRITICAL = 5,
    /// When something goes horribly, horribly wrong.
    FATAL = 6,
    /// Log nothing. Should only be used for setting the level of the logger, because the message would always be logged.
    NONE = 7,
}

#[derive(Clone)]
/// A logger used for logging messages at different levels.
/// Loggers are in a hierarchical structure, so sections of loggers can be turned on and off. 
pub struct Logger {
    inner: Arc<Mutex<logger::Logger>>,
}
impl Logger {
    /// Create a new logger.
    /// 
    /// # Arguments 
    /// 
    /// * `name`: The name of the logger. 
    /// Sub-logger can be created with a dot, so that `logging::Logger::new("foo.bar");` is a sub-logger of `logging::Logger::new("foo");`
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
            inner: logger::Logger::new(name)
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
    /// logger.log("Hello World", Level::INFO);
    /// ```
    pub fn log(&self, msg: impl ToString, level: Level) {
        let locked = self.inner.lock().unwrap();
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
    /// logger.debug("Hello World");
    /// ```
    pub fn debug(&self, msg: impl ToString) {
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
    /// logger.info("Hello World");
    /// ```
    pub fn info(&self, msg: impl ToString) {
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
    /// logger.success("Hello World");
    /// ```
    pub fn success(&self, msg: impl ToString) {
        self.log(msg, Level::SUCCESS)
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
    /// logger.error("Hello World");
    /// ```
    pub fn error(&self, msg: impl ToString) {
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
    /// logger.critical("Hello World");
    /// ```
    pub fn critical(&self, msg: impl ToString) {
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
    /// logger.fatal("Hello World");
    /// ```
    pub fn fatal(&self, msg: impl ToString) {
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
    /// logger.debug("Hello World");
    ///
    /// logger.set_level(Level::INFO);
    /// // will not be logged
    /// logger.debug("Hello World");
    /// // will be logged
    /// logger.info("Hello World");
    /// ```
    /// ```
    /// use logging::Level;
    /// use logging::CONSOLE_HANDLER;
    /// logging::add_handler(&CONSOLE_HANDLER);
    /// let parent = logging::Logger::new("foo");
    /// let child = logging::Logger::new("foo.bar");
    /// parent.set_level(Level::INFO);
    /// // will be logged
    /// child.info("Hello World");
    /// // will not be logged
    /// child.debug("Hello World");
    /// child.set_level(Level::DEBUG);
    /// // will be logged
    /// child.debug("Hello World");
    /// // will not be logged
    /// parent.debug("Hello World")
    /// ```
    pub fn set_level(&self, new_level: Level) {
        let mut locked = self.inner.lock().unwrap();
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
    /// let logger = logging::Logger::new("foo");
    ///
    /// // will do nothing
    /// logger.info("This won't print");
    ///
    /// logging::add_handler(&CONSOLE_HANDLER);
    ///
    /// // now it will print to the console
    /// logger.info("This will print to the console. Maybe even in a coloured output (if you have that feature enabled).")
    /// ```
    pub fn add_handler(&self, handler: &'static impl Handler) {
        let mut locked = self.inner.lock().unwrap();
        locked.add_handler(handler)
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
    /// let logger = Logger::new("foo");
    /// logger.set_level(Level::ALL);
    /// 
    /// // does nothing
    /// logger.info("won't log");
    /// 
    /// logger.add_handler(&ConsoleHandler{});
    /// // will log
    /// logger.info("will print to console");
    /// 
    /// ```
    fn log(&self, level: Level, message: String, logger: String);
}
/// A default implementation of [Handler](Handler).
/// Logs to the console in a potentially coloured output (if you have the coloured_output feature enabled).
pub const CONSOLE_HANDLER: ConsoleHandler = ConsoleHandler {  };
/// The underlying struct of the [CONSOLE_HANDLER](CONSOLE_HANDLER) const.
/// Please use [CONSOLE_HANDLER](CONSOLE_HANDLER) and don't use this struct.
pub struct ConsoleHandler {}
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
/// logger.info("This won't log");
/// 
/// logging::set_level(Level::ALL);
/// // will log.
/// logger.info("This will log");
/// ```
pub fn set_level(level: Level) {
    logger::set_level(level)
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
/// logger.debug("Will log.");
/// logger2.debug("Won't log.");
/// 
/// // adds it to all
/// logging::add_handler(&CONSOLE_HANDLER);
/// logger.debug("Will log twice, as the handler was added twice.");
/// logger2.debug("Will now also log.");
/// ```
pub fn add_handler(handler: &'static impl Handler) {
    logger::add_handler(handler);
}