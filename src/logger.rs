use log::{Record, Level, Metadata};

pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "spawner - {}:{}:{} - {}",
                record.file().unwrap(),
                record.line().unwrap(),
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
