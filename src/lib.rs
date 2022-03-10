//! # Simple Clap Logger
//! 
//! A simple cli logger, which aims to mimic the clap format of error reporting in order to create a seamless cli experience without formatting inconsistencies.
//! 
//! ## Example
//! 
//! ```rust
//! use simple_clap_logger::Logger;
//! use log::Level;
//! 
//! // Initialize the logger with Info logging level
//! Logger::init_with_level(Level::Info);
//! 
//! ```

use log::{Log, Level};
use colored::{Colorize, ColoredString};

/// Main logger struct
pub struct Logger {
    level: Level,
}

impl Logger {
    /// Initializes logger with a standard logging level of [`Level::Error`]
    /// 
    /// # Panics
    /// This function may only be called once in the application, as it registers the logger as global logger. In case any init function is called more than once the application panics.
    pub fn init(){
        Self::init_with_level(Level::Error);
    }

    /// Initializes logger with provided logging level
    /// 
    /// # Panics
    /// This function may only be called once in the application, as it registers the logger as global logger. In case any init function is called more than once the application panics.
    pub fn init_with_level(level: Level){
        let logger = Logger {
            level,
        };

        log::set_boxed_logger(Box::new(logger)).expect("Failed to setup logger, as another one is already registered");
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.level <= record.level() {
            return;
        }

        let prefix = get_prefix(&self.level);

        if self.level == Level::Error {
            eprintln!("{:6} {}", prefix, record.args());
        } else {
            println!("{:6} {}", prefix, record.args());
        }
    }

    fn flush(&self) {}
}

/// Returns a colored prefix for each log level
fn get_prefix(level: &Level) -> ColoredString{
    match level {
        Level::Error => "error:".red().bold(),
        Level::Warn => "warn:".yellow().bold(),
        Level::Info => "info:".green().bold(),
        Level::Debug => "debug:".blue().bold(),
        Level::Trace => "trace:".magenta().bold(),
    }
}

#[cfg(test)]
mod tests {
    use log::{Log, Level, MetadataBuilder};
    use crate::Logger;

    #[test]
    fn check_log_level(){
        let logger = Logger {
            level: Level::Info,
        };

        assert!(logger.enabled(&MetadataBuilder::new().level(Level::Error).build()));
        assert!(logger.enabled(&MetadataBuilder::new().level(Level::Warn).build()));
        assert!(logger.enabled(&MetadataBuilder::new().level(Level::Info).build()));
        assert!(!logger.enabled(&MetadataBuilder::new().level(Level::Debug).build()));
        assert!(!logger.enabled(&MetadataBuilder::new().level(Level::Trace).build()));
    }
}