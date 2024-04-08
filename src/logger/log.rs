use std::io::{self, Write};
use std::time::SystemTime;

pub struct Logger;

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let mut output: Box<dyn Write> = match level {
            LogLevel::Info | LogLevel::Warning => Box::new(io::stdout()),
            LogLevel::Error => Box::new(io::stderr()),
        };

        let level_str = match level {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        };

        let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => duration.as_secs(),
            Err(_) => 0,
        };

        match writeln!(output, "[{}] [{}] {}", timestamp, level_str, message) {
            Ok(_) => (),
            Err(err) => eprintln!("Failed to write to output: {}", err),
        }
    }
    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warn(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
