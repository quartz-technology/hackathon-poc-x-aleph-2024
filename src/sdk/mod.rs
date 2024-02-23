use crate::http::HttpClient;

use self::{post::PostSDK, store::StoreSDK};

pub mod post;
pub mod store;

pub struct AlephSDK<'a> {
    post: PostSDK<'a>,
    store: StoreSDK<'a>,
}

impl <'a>AlephSDK<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        AlephSDK {
            post: PostSDK::new(client),
            store: StoreSDK::new(client),
        }
    }

    pub fn post(&self) -> &PostSDK {
        &self.post
    }

    pub fn store(&self) -> &StoreSDK {
        &self.store
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::HttpClient;

    #[test]
    fn it_builds_the_sdk() {
        let client = HttpClient::new().unwrap();
        let _sdk = AlephSDK::new(&client);
    }
}