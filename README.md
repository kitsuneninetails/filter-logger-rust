# Filter Logger for Rust

This simple logger will filter based on either the module path or the log body 
in each log line, or both.

to use, simply add the crate to your `Cargo.toml`:

```toml
[dependencies]
filter-logger = "*"
log = "*"
```

In your source code, initialize the logger with the module filter and body filter (use 
empty vector for no filter):

```rust
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
```

The output will be:
```text
<date> <time> INFO [test_log::foo1] This will print out
```

The filters are a simple string check on either the `module_path()` or the `args()` parameter 
of the record object passed into the `log()` function.

## Date-time Format

The format can be changed by using the `with_format` function instead of `init`:

```rust
extern crate filter_logger;
#[macro_use] extern crate log;

use filter_logger::FilterLogger;

#[test]
fn test() {
    FilterLogger::with_format(log::Level::Info, vec![], vec![], "%Y%m%dT%H%M%S%z");
    info!("Should use RFC 3339 format!");
}
```

