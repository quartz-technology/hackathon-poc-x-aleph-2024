use crate::http::HttpClient;

pub struct StoreSDKV0<'a> {
    client: &'a HttpClient,
}

impl <'a>StoreSDKV0<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        StoreSDKV0 { client }
    }
}