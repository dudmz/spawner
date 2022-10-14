use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::net::{ToSocketAddrs, TcpStream};
use std::time::Duration;

use lazy_static::lazy_static;
use log::info;
use openssl::ssl::{SslMethod, SslConnector};
use regex::Regex;

use crate::url;

const URL_REGEX: &str = r"https?://[-A-Za-z0-9+&@#/%?=~_()|!:,.;]*[-A-Za-z0-9+&@#/%=~_()|]";

pub struct Crawler {
    start_url: String,
    depth: u8,
    frontier: Vec<String>
}

impl Crawler {
    pub fn new(start_url: String, depth: u8) -> Crawler {
        Crawler {
            start_url,
            depth,
            frontier: Vec::new()
        }
    }

    // TODO: query robots.txt to know which URIs to NOT crawl
    fn request(&self, url: String) -> Result<String, std::io::Error> {
        let ip_addr = match url.to_socket_addrs() {
            Ok(mut data) => data.next().unwrap(),
            Err(error) => return Err(error)
        };
        let hostname = url.split(':').next().unwrap();

        let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
        let stream = TcpStream::connect_timeout(&ip_addr, Duration::from_millis(5000)).unwrap();
        let mut stream = connector.connect(hostname, stream).unwrap();

        let mut headers = HashMap::new();
        headers.insert("Host", hostname);
        headers.insert("Connection", "close");
        let http_header = format!(
            "GET / HTTP/1.1\r\n{}\r\n\r\n",
            headers
                .iter()
                .map(|(key, val)| format!("{}: {}", key, val))
                .collect::<Vec<_>>()
                .join("\r\n")
        );

        stream.write(http_header.as_bytes()).expect("could not write to socket");
        stream.flush().expect("could not flush socket");

        let mut response = String::new();
        stream.read_to_string(&mut response).expect("could not read the response");
        Ok(response)
    }

    // should extract the base website + URIs
    fn extract(&self, data: String) -> HashMap<String, HashSet<String>> {
        let mut res: HashMap<String, HashSet<String>> = HashMap::new();
        lazy_static! {
            static ref RE: Regex = Regex::new(URL_REGEX).unwrap();
        }
        let sample: HashSet<String> = RE.find_iter(data.as_str())
            .into_iter()
            .map(|x| url::base_url(x.as_str().to_string()))
            .collect();

        for url in sample.into_iter() {
            res.insert(url, HashSet::new());
        }

        for mat in RE.find_iter(data.as_str()).filter(|x| url::is_path(x.as_str().to_string())) {
            let urls = res.get_mut(url::base_url(mat.as_str().to_string()).as_str());
            urls.unwrap().insert(mat.as_str().to_string());
        }

        res
    }

    // cycle through depth: depth -> request -> extract -> store -> next depth
    pub fn process(&self) -> Result<Vec<String>, std::io::Error> {
        for _ in 0..self.depth {
            // it should be sync at first
            let data = match self.request(self.start_url.clone()) {
                Ok(data) => data,
                Err(error) => return Err(error)
            };
            for (k, v) in self.extract(data) {
                println!("Domain: {} - urls: {:?}\n", k, v);
            }
        }
        Ok(self.frontier.clone())
    }
}
