use crate::crawler::lib::Crawler;
use crate::web::lib::Web;

pub fn serve(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut web = Web::new();
    match web.listen(url) {
        Ok(()) => {},
        Err(_) => {}
    }
    Ok(())
}

// execute pings web, and starts listening to TCP 6078.
// Incoming messages are crawling requests.
pub fn execute(web_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut krwlr = Crawler::new(String::new(), 1)?;
    krwlr.connect(web_url)?;
    Ok(())
}