use reqwest::Method;
use std::collections::HashMap;
use url::{ParseError, Url};

pub struct Request {
    pub method: Method,
    pub path: String,
    pub query_params: HashMap<String, String>,
}

impl Request {
    pub fn get_url(&self, api_url: &str) -> Result<Url, ParseError> {
        let mut url = Url::parse((api_url.to_string() + self.path.as_str()).as_str())?;
        let query_params = self.query_params.iter().map(|(key, value)| {
            format!("{}={}", key, value)
        }).collect::<Vec<String>>().join("&");

        if !query_params.is_empty() {
            url.set_query(Some(query_params.as_str()));
        }

        Ok(url)
    }
}