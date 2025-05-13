#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::Logger::new(module_path!()).log(format!($($arg)*), $level)
    };
    ($logger:expr => $level:expr, $($arg:tt)*) => {
        $logger.log(format!($($arg)*), $level)
    };
}
#[macro_export]
macro_rules! debug {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::DEBUG, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::DEBUG => $($arg)*)
    };
}
#[macro_export]
macro_rules! info {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::INFO, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::INFO => $($arg)*)
    };
}
#[macro_export]
macro_rules! success {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::SUCCESS, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::SUCCESS => $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::WARN, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::WARN => $($arg)*)
    };
}
#[macro_export]
macro_rules! error {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::ERROR, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::ERROR => $($arg)*)
    };
}
#[macro_export]
macro_rules! critical {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::CRITICAL, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::CRITICAL => $($arg)*)
    };
}
#[macro_export]
macro_rules! fatal {
    ($logger:expr => $($arg:tt)*) => {
        $crate::log!($logger => $crate::Level::FATAL, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log!($crate::level::FATAL => $($arg)*)
    };
}