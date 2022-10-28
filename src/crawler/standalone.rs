use log::{info, error};

use crate::crawler::crawler::Crawler;

// execute starts the loop execution of the crawler through `process`
//
pub fn execute(start_url: String) -> Result<Vec<(String, String)>, Vec<Box<dyn std::error::Error + Send + Sync>>> {
    let mut krwlr = Crawler::new(start_url, 2);
    match krwlr.process() {
        Ok(data) => {
            info!("data: {:?}", data);
            return Ok(data);
        },
        Err(error) => {
            error!("error when executing standalone crawler: {:?}", error);
            return Err(error);
        }
    };
}
