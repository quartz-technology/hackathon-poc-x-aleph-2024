use crate::http::HttpClient;

pub struct PostSDKV0<'a> {
    client: &'a HttpClient,
}

impl <'a>PostSDKV0<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        PostSDKV0 { client }
    }
}