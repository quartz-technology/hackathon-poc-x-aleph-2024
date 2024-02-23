use crate::http::HttpClient;

use self::v0::StoreSDKV0;

pub mod v0;

pub struct StoreSDK<'a> {
    v0: StoreSDKV0<'a>,
}

impl <'a>StoreSDK<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        StoreSDK {
            v0: StoreSDKV0::new(client),
        }
    }

    pub fn v0(&self) -> &StoreSDKV0 {
        &self.v0
    }
}