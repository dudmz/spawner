use crate::web::lib::Web;

pub fn serve() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut web = Web::new();
    match web.listen() {
        Ok(()) => {},
        Err(error) => {}
    }
    Ok(())
}
