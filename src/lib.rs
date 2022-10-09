use std::process;

use log;

pub enum CrawlerMode {
    STANDALONE,
    DISTRIBUTED,
    INVALID
}

pub fn exit(msg: &str) {
    log::error!("{}", msg);
    process::exit(0x0100);
}
