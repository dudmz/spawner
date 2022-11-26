use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{Read, Write};
use std::net::ToSocketAddrs;
use std::sync::{Mutex, Arc};
use std::thread;

use log::error;

use crate::net;
use crate::url;

pub enum CrawlerMode {
    STANDALONE,
    DISTRIBUTED,
    INVALID
}

trait Request {
    fn request(&self, url: String, path: String) -> Result<String, Box<dyn Error + Send + Sync>>;
}

pub struct CrawlerInner {
    start_url: String,
    seen: Arc<Mutex<HashMap<String, HashSet<String>>>>
}

#[derive(Clone)]
pub struct Crawler {
    depth: u8,
    frontier: Vec<(String, String)>,
    inner: Arc<Mutex<CrawlerInner>>
}

impl Request for CrawlerInner {
    // request prepares and send an HTTP request to the domain requesting a determined URI
    fn request(&self, domain: String, uri: String) -> Result<String, Box<dyn Error + Send + Sync>> {
        let ip_addr = domain.to_socket_addrs()?.next().unwrap();
        let hostname = domain.split(':').next().unwrap();
        let mut ssl_stream = net::build_ssl_stream(hostname, ip_addr)?;

        let mut headers = HashMap::new();
        headers.insert("Host", hostname);
        headers.insert("Connection", "close");
        let http_header = format!(
            "GET {} HTTP/1.1\r\n{}\r\n\r\n",
            uri,
            headers
                .iter()
                .map(|(key, val)| format!("{}: {}", key, val))
                .collect::<Vec<_>>()
                .join("\r\n")
        );

        let mut response = String::new();
        ssl_stream.write(http_header.as_bytes()).expect("could not write to socket");
        ssl_stream.flush().expect("could not flush socket");
        ssl_stream.read_to_string(&mut response).expect("could not read the response");

        Ok(response)
    }
}

impl Crawler {
    pub fn new(start_url: String, depth: u8) -> Crawler {
        Self {
            depth,
            frontier: Vec::new(),
            inner: Arc::new(Mutex::new(CrawlerInner {
                start_url,
                seen: Arc::new(Mutex::new(HashMap::new()))
            }))
        }
    }

    // extract_urls extracts the "base_url:port" from the crawler response, along with their
    // URI's that are cited, and filters already seen domains and path combinations
    fn extract_urls(&mut self, url_data: &str, frontier: &Arc<Mutex<Vec<(String, String)>>>) {
        for (k, v) in url::extract(url_data.to_string()) {
            if let Ok(local_self) = self.inner.lock() {
                if let Ok(mut seen) = local_self.seen.lock() {
                    if seen.contains_key(k.as_str()) {
                        for url in v {
                            if !seen.get_mut(k.as_str()).unwrap().contains(url.as_str()) {
                                seen.get_mut(k.as_str()).unwrap().insert(url.to_string());
                                if let Ok(mut frontier) = frontier.lock() {
                                    frontier.push(url::format(url.to_string()));
                                }
                            }
                        }
                    } else {
                        seen.insert(k.to_string(), v.clone());
                        if let Ok(mut frontier) = frontier.lock() {
                            frontier.extend(v.iter().map(|x| url::format(x.to_string())));
                        }
                    }
                }
            }
        }
    }

    // evaluate_urls ranks the domains and paths references through each iteration
    // TODO: Evaluate domains and rank them according to number of references in each iteration
    fn evaluate_urls(&mut self) {}

    pub fn connect(&mut self, web_url: String) {}

    // process executes the crawling cycle through determined depth
    // process -> depth -> frontier -> spawn requests -> extract -> eval -> store -> next depth
    pub fn process(&mut self) -> Result<Vec<(String, String)>, Vec<Box<dyn Error + Send + Sync>>> {
        for i in 0..self.depth {
            // in the first depth, it should be sync
            if i == 0 {
                let data: String;
                if let Ok(local_self) = self.inner.lock() {
                    data = match local_self.request(local_self.start_url.clone(), "/".to_string()) {
                        Ok(data) => data,
                        Err(error) => {
                            let mut errors: Vec<Box<dyn Error + Send + Sync>> = vec![];
                            errors.push(error);
                            return Err(errors)
                        }
                    };

                    if let Ok(mut seen) = local_self.seen.lock() {
                        for (k, v) in url::extract(data) {
                            seen.insert(k.to_string(), v.clone());
                            let to_frontier: Vec<(String, String)> = v.iter()
                                .map(|url| url::format(url.to_string()))
                                .collect();
                            self.frontier.extend(to_frontier);
                        }
                    }
                }
            } else {
                let mut threads: Vec<thread::JoinHandle<()>> = vec![];
                let errors: Arc<Mutex<Vec<Box<dyn Error + Send + Sync>>>> = Arc::new(Mutex::new(vec![]));
                let new_frontier: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(vec![]));

                for url in self.frontier.clone() {
                    let errors = Arc::clone(&errors);
                    let frontier = Arc::clone(&new_frontier);
                    let mut crawler = self.clone();

                    let child_thread = thread::spawn(move || {
                        let mut url_data: String = String::from("");
                        if let Ok(local_self) = crawler.inner.lock() {
                            match local_self.request(url.0, url.1) {
                                Ok(data) => {
                                    url_data = data;
                                },
                                Err(error) => {
                                    if let Ok(mut err) = errors.lock() {
                                        err.push(error);
                                    }
                                }
                            }
                        }

                        if !url_data.is_empty() {
                            crawler.extract_urls(url_data.as_str(), &frontier);
                        }
                    });

                    threads.push(child_thread);
                }

                for child in threads {
                    match child.join() {
                        Ok(_) => {},
                        Err(error) => error!("thread error: {:?}", error)
                    }
                }

                if let Ok(frontier) = new_frontier.clone().lock() {
                    self.frontier.extend(frontier.clone().into_iter());
                }
            }
        }

        Ok(self.frontier.clone())
    }
}

