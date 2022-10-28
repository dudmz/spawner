use std::process;

use log;

pub fn exit(msg: &str) {
    log::error!("{}", msg);
    process::exit(0x0100);
}

