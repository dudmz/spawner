use log::{info, error};

use crate::crawler::crawler::Crawler;

// execute starts the loop execution of the crawler through `process`
//
pub fn execute(start_url: String) {
    let mut krwlr = Crawler::new(start_url, 2);
    match krwlr.process() {
        Ok(data) => info!("data: {:?}", data),
        Err(error) => error!("error when executing standalone crawler: {:?}", error)
    };
}
