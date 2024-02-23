use crate::http::HttpClient;

use self::v0::PostSDKV0;

pub mod v0;

pub struct PostSDK<'a> {
    v0: PostSDKV0<'a>,
}

impl <'a>PostSDK<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        PostSDK {
            v0: PostSDKV0::new(client),
        }
    }

    pub fn v0(&self) -> &PostSDKV0 {
        &self.v0
    }
}