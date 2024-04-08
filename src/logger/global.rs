use lazy_static::lazy_static;
use std::sync::Mutex;

use super::log::Logger;

lazy_static! {
    pub static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new());
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::logger::global::LOGGER.lock().unwrap().info(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::logger::global::LOGGER.lock().unwrap().warn(&format!($($arg)*));
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::logger::global::LOGGER.lock().unwrap().error(&format!($($arg)*));
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_logger() {
        info!("info message");
        warn!("warn message");
        error!("error message");
    }
}
