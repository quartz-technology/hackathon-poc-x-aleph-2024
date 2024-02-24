use reqwest::{Client, ClientBuilder, Response, StatusCode};
use serde::Serialize;
use thiserror::Error;
use url::{ParseError, Url};

use super::request::Request;

#[derive(Debug, Error)]
pub enum HttpClientError {
    #[error("failed to perform request: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("failed to parse api url: {0}")]
    ApiUrlParseError(#[from] ParseError),

    #[error("api url must not have a trailing slash")]
    ApiUrlHasTrailingSlash,

    #[error("response returned error {status}: {data}")]
    ResponseError { status: StatusCode, data: String },
}

pub struct HttpClient {
    pub api_url: String,
    pub requester: Client,
}

impl HttpClient {
    pub fn new() -> Result<Self, HttpClientError> {
        let builder = ClientBuilder::new();
        let requester = builder.build()?;

        Ok(HttpClient {
            api_url: "https://api2.aleph.im".to_string(),
            requester,
        })
    }

    pub fn with_api_url(mut self, api_url: &str) -> Result<Self, HttpClientError> {
        Url::parse(api_url)?;

        match api_url.ends_with('/') {
            true => Err(HttpClientError::ApiUrlHasTrailingSlash),
            false => {
                self.api_url = api_url.to_owned();
                Ok(self)
            }
        }
    }

    pub fn with_requester(mut self, requester: Client) -> Self {
        self.requester = requester;
        self
    }

    pub async fn do_request(&self, req: Request) -> Result<Response, HttpClientError> {
        let url = req.get_url(self.api_url.as_str())?;
        let mut http_req = reqwest::Request::new(req.method, url);

        http_req.headers_mut().insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_static("application/json"),
        );

        if let Some(body) = req.body {
            http_req.body_mut().replace(body.into());
        }

        let res = self.requester.execute(http_req).await?;

        match res.status().as_u16() < 200 || res.status().as_u16() >= 300 {
            true => {
                let status = res.status();
                let data = res.text().await?;

                Err(HttpClientError::ResponseError { status, data })
            }
            false => Ok(res),
        }
    }

    pub async fn do_post_request<T: Serialize>(&self, req: Request, body: T) -> Result<Response, HttpClientError> {
        let url = req.get_url(self.api_url.as_str())?;

        let res = reqwest::Client::new()
            .post(url)
            .json(&body)
            .send()
            .await?;

        match res.status().as_u16() < 200 || res.status().as_u16() >= 300 {
            true => {
                let status = res.status();
                let data = res.text().await?;

                Err(HttpClientError::ResponseError { status, data })
            }
            false => Ok(res),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_the_http_client() {
        let client = HttpClient::new();

        assert_eq!(true, client.is_ok());
    }

    #[test]
    fn it_builds_the_http_client_with_api_url() {
        let client = HttpClient::new()
            .unwrap()
            .with_api_url("https://api2.aleph.im");

        assert_eq!(true, client.is_ok());
    }

    #[test]
    fn it_fails_to_build_the_http_client_with_invalid_api_url() {
        let client = HttpClient::new().unwrap().with_api_url("");

        assert_eq!(true, client.is_err());
    }

    #[test]
    fn it_fails_to_build_the_http_client_with_malformed_api_url() {
        let client = HttpClient::new()
            .unwrap()
            .with_api_url("https://api2.aleph.im/");

        assert_eq!(true, client.is_err());
    }
}