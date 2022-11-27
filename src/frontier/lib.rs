use std::sync::{Arc, Mutex};
use std::time::SystemTime;

pub struct Frontier {
    pub data: Arc<Mutex<Contents>>,
    last_feed: SystemTime
}

pub struct Contents {
    pub contents: Vec<(String, String)>
}

impl Frontier {
    pub fn new() -> Frontier {
        Self {
            data: Arc::new(Mutex::new(Contents::new())),
            last_feed: SystemTime::now()
        }
    }

    pub fn append(&mut self, data: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut self_data) = self.data.lock() {
            self_data.contents.extend(data);
        }
        Ok(())
    }

    pub fn sync(&mut self, data: Vec<(String, String)>) -> Result<(), Box<dyn std::error::Error>> {
        if let Ok(mut self_data) = self.data.lock() {
            self_data.contents = data;
        }
        Ok(())
    }

    pub fn export_to_file() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

impl Contents {
    fn new() -> Contents {
        Self {
            contents: vec![]
        }
    }
}
