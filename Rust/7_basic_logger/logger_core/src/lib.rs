pub mod color;

use crate::color::ColoredStr;
use log::{LevelFilter, Log};
use time::{format_description, OffsetDateTime};

cfg_if::cfg_if! {
    if #[cfg(feature = "file")] {
        use std::io::{BufWriter, Write};
        use std::fs::OpenOptions;
    }
}

pub struct Logger {
    level: LevelFilter,
    ts_format: String,
    colored: bool,

    #[cfg(feature = "file")]
    log_path: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            level: LevelFilter::Trace,
            ts_format: "[hour]:[minute]:[second].[subsecond digits:3]".to_string(),
            colored: false,
            #[cfg(feature = "file")]
            log_path: "..".to_string(),
        }
    }

    pub fn colored(mut self) -> Self {
        self.colored = true;
        self
    }

    pub fn ts_format(mut self, f: &str) -> Self {
        self.ts_format = f.to_string();
        self
    }

    pub fn init(self) -> Result<(), log::SetLoggerError> {
        log::set_max_level(self.level);
        log::set_boxed_logger(Box::new(self))?;
        Ok(())
    }

    pub fn warn(msg: &str) {
        log::warn!("{}", msg);
    }

    pub fn debug(msg: &str) {
        log::debug!("{}", msg);
    }

    pub fn err(msg: &str) {
        log::error!("{}", msg);
    }

    pub fn info(msg: &str) {
        log::info!("{}", msg);
    }

    pub fn trace(msg: &str) {
        log::trace!("{}", msg);
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let formattable =
                format_description::parse(&self.ts_format).expect("Fail to format time string.");
            let timestamp = format!(
                "{}",
                OffsetDateTime::now_local()
                    .expect("Fail to get local time")
                    .format(&formattable)
                    .unwrap_or_default()
            );

            let target = if record.target().is_empty() {
                record.module_path().unwrap_or_default()
            } else {
                record.target()
            };

            let mut message = format!("{}: {}", record.level(), record.args());
            if self.colored {
                let mut colored_str = ColoredStr::from(message);
                message = match record.level().to_level_filter() {
                    LevelFilter::Error => colored_str.bright().color(color::Color::Red).build(),
                    LevelFilter::Warn => colored_str.bright().color(color::Color::Yellow).build(),
                    LevelFilter::Debug => colored_str.bright().build(),
                    LevelFilter::Info => colored_str.bold().build(),
                    _ => colored_str.build(),
                }
            }
            println!("[{}]@{} {}", timestamp, target, message);
        }
    }

    fn flush(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::color::{Color, ColoredStr};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_color_value() {
        assert_eq!(Color::Red as u8, 31);
    }

    #[test]
    fn test_color() {
        let colored_string = ColoredStr::new()
            .content("a red string")
            .color(Color::Red)
            .build();
        assert_eq!(colored_string, "\x1b[31ma red string\x1b[0m");
    }
}
