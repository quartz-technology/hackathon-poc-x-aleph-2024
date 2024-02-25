use crate::http::HttpClient;

use self::{post::PostSDK, store::StoreSDK};

pub mod post;
pub mod store;
pub mod common;

pub struct AlephSDK {
    post: PostSDK,
}

impl AlephSDK {
    pub fn new(client: HttpClient) -> Self {
        AlephSDK {
            post: PostSDK::new(client),
        }
    }

    pub fn post(&self) -> &PostSDK {
        &self.post
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpClient;

    #[test]
    fn it_builds_the_sdk() {
        let client = HttpClient::new().unwrap();
        let _sdk = AlephSDK::new(client);
    }
}