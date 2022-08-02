//! # Simple logger
//!
//! `Simple logger` is a library logs by a level of importance<br>
//! The funcions allowed are
//! - trace
//! - debug
//! - info
//! - warn
//! - error
//!
//! # References
//! <table>
//! <thead>
//! <tr>
//! <th>Level of request</th>
//! <th>TRACE</th>
//! <th>DEBUG</th>
//! <th>INFO</th>
//! <th>WARN</th>
//! <th>ERROR</th>
//! <th>OFF</th>
//! </tr>
//! </thead>
//! <tbody>
//! <tr>
//! <td>TRACE</td>
//! <td>YES</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! </tr>
//! <tr>
//! <td>DEBUG</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! </tr>
//! <tr>
//! <td>INFO</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>NO</td>
//! <td>NO</td>
//! <td>NO</td>
//! </tr>
//! <tr>
//! <td>WARN</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>NO</td>
//! <td>NO</td>
//! </tr>
//! <tr>
//! <td>ERROR</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>YES</td>
//! <td>NO</td>
//! </tr>
//! </tbody>
//! </table>
//!
//! All of this methods received a vector of strings
//! # Examples
//! ```
//! let logger = Logger::new(Levels::INFO)
//! logger.info(vec![
//!     String::from("Some"),
//!     String::from("undefined"),
//!     String::from("strings"),
//!     String::from("to"),
//!     String::from("print")
//! ])
//! ```
//! **Output**<br>
//! ```
//! [ INFO ] Some undefined strings to print // At level info just INFO, WARN, ERROR logs are printed
//!```
use core::fmt;
use std::fmt::Display;

/// Logger struct
pub struct Logger {
    pub level: u8,
}

/// ENUM to define witch level of loggs can print
pub enum Levels {
    TRACE = 37,
    DEBUG = 36,
    INFO = 34,
    WARN = 33,
    ERROR = 31,
    DEFAULT = 30,
}

/// Getting the name of the enum
impl Display for Levels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Levels::TRACE => write!(f, "TRACE"),
            Levels::DEBUG => write!(f, "DEBUG"),
            Levels::INFO => write!(f, "INFO"),
            Levels::WARN => write!(f, "WARN"),
            Levels::ERROR => write!(f, "ERROR"),
            Levels::DEFAULT => write!(f, "DEFAULT"),
        }
    }
}

fn printer(logger_level: u8, _type: Levels, text: Vec<String>) {
    let name = _type.to_string();
    let level = _type as u8;
    if logger_level >= 31 && level <= logger_level {
        println!(
            "\x1b[{}m\x1b[1m[ {} ]\x1b[0m {}",
            level,
            name,
            text.join(" ")
        );
    }
}

impl Logger {
    pub fn new(level: Levels) -> Self {
        Self { level: level as u8 }
    }
    pub fn trace(&self, texts: Vec<String>) {
        printer(self.level, Levels::TRACE, texts);
    }
    pub fn debug(&self, texts: Vec<String>) {
        printer(self.level, Levels::DEBUG, texts);
    }
    pub fn info(&self, texts: Vec<String>) {
        printer(self.level, Levels::INFO, texts);
    }
    pub fn warn(&self, texts: Vec<String>) {
        printer(self.level, Levels::WARN, texts);
    }
    pub fn error(&self, texts: Vec<String>) {
        printer(self.level, Levels::ERROR, texts);
    }
}

#[cfg(test)]
mod tests {
    use crate::Levels;
    use crate::Logger;

    #[test]
    fn get_logger_level() {
        let logger = Logger::new(Levels::INFO);
        assert_eq!(logger.level, Levels::INFO as u8);
    }
}
