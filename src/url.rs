use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use regex::Regex;

pub fn base_url(url: String) -> Option<String> {
    lazy_static! {
        static ref BASE_URL_RE: Regex = Regex::new(r"https?://[-A-Za-z0-9+&@#%?=~_()|!:,.;]*/").unwrap();
    }
    match BASE_URL_RE.find(url.as_str()) {
        Some(url) => Some(url.as_str().to_string()),
        None => Some("".to_string())
    }
}

pub fn format(url: String) -> (String, String) {
    lazy_static! {
        static ref GET_BASE_PATH: Regex = Regex::new(r"^(https?)://([-A-Za-z0-9+&@#%=~_()|!:,.;]*)(/[-A-Za-z0-9+&@#/%=~_()|]*)$").unwrap();
    }
    let caps = GET_BASE_PATH.captures(url.as_str()).unwrap();
    let server = format!(
        "{}:{}",
        caps.get(2).map_or("", |s| s.as_str()),
        caps.get(1).map_or("80", |s| {
            if s.as_str() == "https" {
                "443"
            } else {
                "80"
            }
        })
    );
    let uri = format!("{}", caps.get(3).map_or("", |s| s.as_str()));

    (server, uri)
}

pub fn is_path(url: String) -> bool {
    lazy_static! {
        static ref IS_PATH_RE: Regex = Regex::new(r"^https?://[-A-Za-z0-9+&@#%=~_()|!:,.;]*/[-A-Za-z0-9+&@#/%=~_()|]*$").unwrap();
    }
    IS_PATH_RE.is_match(url.as_str())
}

pub fn is_asset(url: String) -> bool {
    lazy_static! {
        static ref IS_ASSET_RE: Regex = Regex::new(r"https?://[-A-Za-z0-9+&@#%=~_()|/!:,.;]*/[-A-Za-z0-9+&@#/%=~_()|]*\.[a-z]*").unwrap();
    }
    IS_ASSET_RE.is_match(url.as_str())
}

pub fn extract(data: String) -> HashMap<String, HashSet<String>> {
    let mut res: HashMap<String, HashSet<String>> = HashMap::new();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"https?://[-A-Za-z0-9+&@#/%?=~_()|!:,.;]*[-A-Za-z0-9+&@#/%=~_()|]").unwrap();
    }
    let sample: HashSet<String> = RE.find_iter(data.as_str())
        .into_iter()
        .filter(|x| x.as_str() != "")
        .map(|x| base_url(x.as_str().to_string()).unwrap())
        .collect();

    for url in sample.into_iter() {
        res.insert(url, HashSet::new());
    }

    for mat in RE.find_iter(data.as_str()).filter(|x| is_path(x.as_str().to_string())) {
        let urls = res.get_mut(base_url(mat.as_str().to_string()).unwrap().as_str());
        urls.unwrap().insert(mat.as_str().to_string());
    }

    res
}

