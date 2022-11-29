use log::{info, error};

use crate::crawler::lib::Crawler;
use crate::frontier::lib::Frontier;
use crate::errors::StandaloneServeUnreachableError;

// execute starts the loop execution of the crawler through `process`
// request -> extract -> store -> feed -> request
pub fn execute(start_url: String) -> Result<Vec<(String, String)>, Box<dyn std::error::Error + Send + Sync>> {
    let mut krwlr = Crawler::new(start_url, 2);
    let mut front = Frontier::new();
    match krwlr.process() {
        Ok(data) => {
            front.sync(data.clone());
            info!("data: {:?}", data);
        },
        Err(error) => {
            error!("error when executing standalone crawler: {:?}", error);
        }
    };

    if let Ok(frontier_data) = front.data.clone().lock() {
        Ok(frontier_data.contents.clone())
    } else {
        Err(StandaloneServeUnreachableError.into())
    }
}
