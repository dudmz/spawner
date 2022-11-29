use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::net::{TcpStream, TcpListener};
use std::sync::{Arc, Mutex};
use std::thread;

use log::{info, warn, error};

pub enum AssignAlgorithm {
     RoundRobin,
     LeastTask
}

#[derive(Clone)]
pub struct Web {
    inner: Arc<Mutex<WebInner>>
}

pub struct WebInner {
    crawlers: HashMap<i64, TcpStream>,
    listener: Option<TcpListener>,
    assigner: AssignAlgorithm,
    pub workers_no: i64,
}

impl Web {
    pub fn new() -> Web {
        Self {
            inner: Arc::new(Mutex::new(WebInner {
                crawlers: HashMap::new(),
                listener: None,
                assigner: AssignAlgorithm::RoundRobin,
                workers_no: 1
            }))
        }
    }

    pub fn listen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let mut local_self = self.inner.lock().unwrap() {
            local_self.listener = match TcpListener::bind("127.0.0.1:6077") {
                Ok(listener) => Some(listener),
                Err(error) => return Err(error.into())
            };
        };

        let mut web = self.clone();
        let web_thread = thread::spawn(move || {
            if let mut local_self = web.inner.lock().unwrap() {
                loop {
                    match local_self.listener.as_ref().unwrap().accept() {
                        Ok((_socket, addr)) => {
                            // stop registering crawlers if workers_no reached its limit
                            if local_self.workers_no == 0 {
                                break;
                            }

                            let worker_no = local_self.workers_no;
                            local_self.crawlers.insert(worker_no, TcpStream::connect(addr).unwrap());
                            local_self.workers_no -= 1;
                        }
                        Err(error) => {
                            error!("listening to port 6077: {error:?}");
                        }
                    }
                }
            }
        });

        info!("waiting for crawlers to connect");
        match web_thread.join() {
            Ok(_) => {},
            Err(error) => error!("thread error: {:?}", error)
        }
        info!("crawler connections established");

        Ok(())
    }
}
