use std::env;

use log;

mod argparser;
mod lib;
mod logger;

use argparser::Program;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info)).unwrap();

    let args: Vec<String> = env::args().collect();
    let prog: Program = Program::new(&args);


    log::info!(
        target: "program",
        "command: {}, mode: {}",
        prog.command.unwrap(),
        prog.opts.mode.unwrap()
    );
}
