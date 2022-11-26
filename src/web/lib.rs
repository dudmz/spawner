use std::collections::HashMap;
use std::net::{TcpStream, TcpListener};

use log::{info, warn};

pub enum AssignAlgorithm {
     RoundRobin,
     LeastTask
}

pub struct Web {
    crawlers: HashMap<i64, Vec<TcpStream>>,
    listener: Option<TcpListener>,
    assigner: AssignAlgorithm,
    workers_no: i64,
}

impl Web {
    pub fn new() -> Web {
        Self {
            crawlers: HashMap::new(),
            listener: None,
            assigner: AssignAlgorithm::RoundRobin,
            workers_no: 3
        }
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.listener = match TcpListener::bind("127.0.0.1:6077") {
            Ok(listener) => Some(listener),
            Err(error) => return Err(error)
        };

        thread::spawn(|| {
            loop {
                match self.listener.unwrap().accept() {
                    Ok((_socket, addr)) => {
                        // stop registering crawlers if workers_no reached its limit
                        if self.workers_no == 0 {
                            continue
                        }

                        self.crawlers.insert(self.workers_no, TcpStream::connect(addr).unwrap());
                        self.workers_no -= 1;

                    }
                    Err(error) => { error!("listening to port 6077: {error:?}") }
                }
            }
        });

        Ok(())
    }
}
