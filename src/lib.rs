#![doc(html_root_url = "https://docs.rs/capture-logger/0.1.0")]
//! [`log`](https://docs.rs/log/0.4.11/log/) implementation for testing.
//!
//! ## Dependencies
//!
//! ```toml
//! [dev-dependencies]
//! testlog = "0.1"
//! ```
//!
//! ## Example
//!
//! ```
//! use capture_logger::{begin_capture, pop_captured};
//!
//! # fn main() {
//! begin_capture();
//! log::debug!("LOG");
//! assert_eq!(pop_captured().unwrap().message(), "LOG");
//! # }
//! ```
//!
//! ## License
//!
//! Licensed under either of
//! * Apache License, Version 2.0
//!   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.!
use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::Once;

pub use log::Level;


static INITIALIZED: Once = Once::new();

/// Captured log Record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    level: Level,
    message: String,
}

impl Record {
    /// get logged level.
    pub fn level(&self) -> Level {
        self.level
    }

    /// get log message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

thread_local! {
    static CAPTURED: RefCell<Option<VecDeque<Record>>> = RefCell::new(None);
}

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        CAPTURED.with(|cap| {
            if let Some(queue) = &mut *cap.borrow_mut() {
                queue.push_back(Record {
                    level: record.level(),
                    message: format!("{}", record.args()),
                });
            }
        });
    }

    fn flush(&self) {
    }
}

/// Begin capturing log.
///
/// # Panics
///
/// Panics if already begin captured.
pub fn begin_capture() {
    INITIALIZED.call_once(|| {
        log::set_logger(&LOGGER).unwrap();
        log::set_max_level(log::LevelFilter::Trace);
    });

    CAPTURED.with(|cap| {
        let queue = &mut *cap.borrow_mut();
        if queue.is_none() {
            *queue = Some(VecDeque::new());
        } else {
            panic!("already captured.")
        }
    });
}

/// End capturing log.
pub fn end_capture() {
    CAPTURED.with(|cap| {
        *cap.borrow_mut() = None;
    });
}

/// Pop captured log record.
///
/// # Panics
///
/// Panics if no begin capture yet.
pub fn pop_captured() -> Option<Record> {
    CAPTURED.with(|cap| {
        if let Some(queue) = &mut *cap.borrow_mut() {
            queue.pop_front()
        } else {
            panic!("not captured.");
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::Log;

    #[test]
    fn test_logger() {
        LOGGER.flush(); // NOP
    }
}
