extern crate filter_logger;
#[macro_use] extern crate log;

use filter_logger::FilterLogger;

#[test]
fn test() {
    FilterLogger::init(log::Level::Info, vec!["foo2".to_string()], vec!["DON'T PRINT".to_string()]);
    foo1::log_it();
    foo2::log_it();
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
