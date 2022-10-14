use std::env;

use log;

mod argparser;
mod lib;
mod logger;
mod crawler;
mod standalone;
mod url;

use argparser::Program;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger;

fn main() {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info)).unwrap();

    let args: Vec<String> = env::args().collect();
    let prog: Program = Program::new(&args);

    match prog.execute() {
        Ok(_) => {},
        Err(error) => lib::exit(error)
    };
}
