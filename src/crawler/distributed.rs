use log::{error, info};

use crate::web::lib::Web;

pub fn serve(start_url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut web = Web::new();
    match web.listen() {
        Ok(()) => {},
        Err(error) => {}
    }
    Ok(())
}
