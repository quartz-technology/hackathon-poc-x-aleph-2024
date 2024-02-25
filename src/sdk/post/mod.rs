use crate::http::HttpClient;

use self::v0::*;

pub mod v0;

pub struct PostSDK {
    v0: PostSDKV0,
}

impl PostSDK {
    pub fn new(client: HttpClient) -> Self {
        PostSDK {
            v0: PostSDKV0::new(client),
        }
    }

    pub fn v0(&self) -> &PostSDKV0 {
        &self.v0
    }
}