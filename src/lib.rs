extern crate chrono;
#[macro_use] extern crate runtime_fmt;
#[macro_use] extern crate log;

use std::boxed::Box;

#[derive(Clone)]
pub struct FilterLogger {
    level: log::Level,
    filter: Vec<String>,
    body_filter: Vec<String>,
}

impl FilterLogger
{
    pub fn init(level: log::Level,
                filter: Vec<String>,
                body_filter: Vec<String>) -> Self {
        let logger = FilterLogger {
            level,
            filter,
            body_filter,
        };

        let _ = log::set_boxed_logger(Box::new(logger.clone()));
        log::set_max_level(level.to_level_filter());

        logger
    }
}

impl log::Log for FilterLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        let now = chrono::Utc::now();
        let skip = record.module_path()
            .map(|m| {
                self.filter.iter()
                    .filter(|f| m.contains(*f))
                    .next()
                    .is_some()
            })
            .unwrap_or(false);
        let msg_str = format!("{}", record.args());
        let body_skip =
            self.body_filter.iter()
                .filter(|f| msg_str.contains(*f))
                .next()
                .is_some();
        if !skip && !body_skip {
            rt_println!("{time} {level} [{module}] {body}",
                        time = now.format("%Y-%m-%d %H:%M:%S%z"),
                        level = record.level(),
                        module = record.module_path().unwrap_or("default"),
                        body = msg_str).unwrap();
        }
    }

    fn flush(&self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test1 {
        pub fn log_it() {
            debug!("test debug");
            info!("test info");
            trace!("test trace");
            error!("test error");
        }
    }

    mod test2 {
        pub fn log_it() {
            debug!("test debug");
            info!("test info");
            trace!("test trace");
            error!("test error");
        }
    }

    #[test]
    fn test_log() {
        FilterLogger::init(log::Level::Trace, vec![], vec![]);
        println!("test_log = 8 Goods Expected (INFO, ERROR, DEBUG, TRACE)");
        test1::log_it();
        test2::log_it();
    }

    #[test]
    fn test_level() {
        FilterLogger::init(log::Level::Info, vec![], vec![]);
        println!("test_level = 4 Goods Expected (INFO and ERROR)");
        test1::log_it();
        test2::log_it();
    }

    #[test]
    fn test_filter() {
        FilterLogger::init(log::Level::Info, vec!["test2".to_string()], vec!["test error".to_string()]);
        println!("test_filter = 1 Good Expected (INFO, module='test1')");
        test1::log_it();
        test2::log_it();
    }
}
