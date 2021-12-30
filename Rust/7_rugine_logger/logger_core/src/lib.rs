
use log::{Log, Level};

cfg_if::cfg_if! {
    if #[cfg(feature = "file")] {
        use std::io::{BufWriter, Write};
        use std::fs::OpenOptions;
    }
}

pub struct Logger {
    min_level: Level,
    ts_format: String,
    colored: bool,

    #[cfg(feature = "file")]
    log_path: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            min_level: Level::Trace,
            ts_format: "MM-dd-yyyy HH:mm:ss.sss".to_string(),
            colored: true,

            #[cfg(feature = "file")]
            log_path: "..".to_string(),
        }
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.min_level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) { }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
