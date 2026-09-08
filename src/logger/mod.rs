use env_logger::Builder;
use log::LevelFilter;

pub struct Logger;

impl Logger {
    pub fn init() {
        Builder::new()
            .filter_level(LevelFilter::Info)
            .format_timestamp_millis()
            .init();
    }
}
