use std::env;

use log;

mod argparser;
mod crawler;
mod errors;
mod frontier;
mod grpc;
mod lib;
mod logger;
mod net;
mod url;

use argparser::Program;

static LOGGER: logger::SimpleLogger = logger::SimpleLogger;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info)).unwrap();

    let args: Vec<String> = env::args().collect();
    let prog: Program = Program::new(&args);

    match prog.execute() {
        Ok(_) => Ok(()),
        Err(error) => Err(error)
    }
}
