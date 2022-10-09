use log::error;

use crate::crawler::Crawler;

pub fn execute(start_url: String) {
    let krwlr = Crawler::new(start_url, 1);
    match krwlr.process() {
        Ok(_) => {},
        Err(error) => error!("error when executing standalone crawler: {:?}", error)
    };
}
