extern crate filter_logger;
#[macro_use]
extern crate log;

use filter_logger::FilterLogger;

#[test]
fn test_log() {
    FilterLogger::init(
        log::Level::Info,
        vec!["foo2".to_string(), "foo3".to_string()],
        vec!["DON'T PRINT".to_string()],
    );
    foo1::log_it();
    foo2::log_it();
    foo3::log_it();
}

#[test]
fn test_format() {
    FilterLogger::with_format(
        log::Level::Info,
        vec![],
        vec![],
        "[year][month][day]T[hour][minute][second]".into(),
    );
    info!("Test logger");
}

mod foo1 {
    pub fn log_it() {
        info!("This will print out");
        info!("DON'T PRINT - This will NOT print out");
    }
}

mod foo2 {
    pub fn log_it() {
        info!("This will NOT print out");
    }
}

mod foo3 {
    pub fn log_it() {
        info!("This will NOT print out");
    }
}
