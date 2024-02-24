use std::collections::HashMap;

use serde::Serialize;

use crate::sdk::common::{BaseMessage, DefaultEthereumSigner, ItemType, MessageSigner};

pub struct ListPostsRequest {
    pub pagination: u32,
    pub page: u32,
    pub custom_types: Option<Vec<String>>,
    pub references: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
    pub hashes: Option<Vec<String>>,
    pub channels: Option<Vec<String>>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
}

impl Default for ListPostsRequest {
    fn default() -> Self {
        ListPostsRequest {
            pagination: 200,
            page: 1,
            custom_types: None,
            references: None,
            addresses: None,
            hashes: None,
            channels: None,
            start_date: None,
            end_date: None,
        }
    }
}

impl ListPostsRequest {
    pub fn with_pagination(mut self, pagination: u32) -> Self {
        self.pagination = pagination;
        self
    }

    pub fn with_page(mut self, page: u32) -> Self {
        self.page = page;
        self
    }

    pub fn with_custom_types(mut self, custom_types: Vec<String>) -> Self {
        self.custom_types = Some(custom_types);
        self
    }

    pub fn with_references(mut self, references: Vec<String>) -> Self {
        self.references = Some(references);
        self
    }

    pub fn with_addresses(mut self, addresses: Vec<String>) -> Self {
        self.addresses = Some(addresses);
        self
    }

    pub fn with_hashes(mut self, hashes: Vec<String>) -> Self {
        self.hashes = Some(hashes);
        self
    }

    pub fn with_channels(mut self, channels: Vec<String>) -> Self {
        self.channels = Some(channels);
        self
    }

    pub fn with_start_date(mut self, start_date: i64) -> Self {
        self.start_date = Some(start_date);
        self
    }

    pub fn with_end_date(mut self, end_date: i64) -> Self {
        self.end_date = Some(end_date);
        self
    }

    pub fn query_params(self) -> HashMap<String, String> {
        let mut query_params = HashMap::new();

        query_params.insert("pagination".to_string(), self.pagination.to_string());
        query_params.insert("page".to_string(), self.page.to_string());

        if let Some(custom_types) = self.custom_types {
            query_params.insert("types".to_string(), custom_types.join(","));
        }

        if let Some(references) = self.references {
            query_params.insert("refs".to_string(), references.join(","));
        }

        if let Some(addresses) = self.addresses {
            query_params.insert("addresses".to_string(), addresses.join(","));
        }

        if let Some(hashes) = self.hashes {
            query_params.insert("hashes".to_string(), hashes.join(","));
        }

        if let Some(channels) = self.channels {
            query_params.insert("channels".to_string(), channels.join(","));
        }

        if let Some(start_date) = self.start_date {
            query_params.insert("start_date".to_string(), start_date.to_string());
        }

        if let Some(end_date) = self.end_date {
            query_params.insert("end_date".to_string(), end_date.to_string());
        }

        query_params
    
    }
}

pub struct CreatePostRequest<T> {
    pub signer: DefaultEthereumSigner,
    pub custom_type: String,
    pub content: T,
    pub channel: String,
    pub item_type: ItemType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_the_list_posts_request_using_custom_values() {
        let request = ListPostsRequest::default()
            .with_pagination(100)
            .with_page(2)
            .with_custom_types(vec!["custom_type".to_string()])
            .with_references(vec!["reference".to_string()])
            .with_addresses(vec!["address".to_string()])
            .with_hashes(vec!["hash".to_string()])
            .with_channels(vec!["channel".to_string()])
            .with_start_date(0)
            .with_end_date(1);

        assert_eq!(request.pagination, 100);
        assert_eq!(request.page, 2);
        assert_eq!(request.custom_types.unwrap().len(), 1);
        assert_eq!(request.references.unwrap().len(), 1);
        assert_eq!(request.addresses.unwrap().len(), 1);
        assert_eq!(request.hashes.unwrap().len(), 1);
        assert_eq!(request.channels.unwrap().len(), 1);
        assert_eq!(request.start_date.unwrap(), 0);
        assert_eq!(request.end_date.unwrap(), 1);
    }

    #[test]
    fn it_builds_the_list_posts_request_using_default_values() {
        let request = ListPostsRequest::default();

        assert_eq!(request.pagination, 200);
        assert_eq!(request.page, 1);
        assert!(request.custom_types.is_none());
        assert!(request.references.is_none());
        assert!(request.addresses.is_none());
        assert!(request.hashes.is_none());
        assert!(request.channels.is_none());
        assert!(request.start_date.is_none());
        assert!(request.end_date.is_none());
    }

    #[test]
    fn it_builds_the_list_posts_request_query_params() {
        let request = ListPostsRequest::default()
            .with_pagination(100)
            .with_page(2)
            .with_custom_types(vec!["custom_type".to_string()])
            .with_references(vec!["reference".to_string()])
            .with_addresses(vec!["address".to_string()])
            .with_hashes(vec!["hash".to_string()])
            .with_channels(vec!["channel".to_string()])
            .with_start_date(0)
            .with_end_date(1);

        let query_params = request.query_params();

        assert_eq!(query_params.get("pagination").unwrap(), "100");
        assert_eq!(query_params.get("page").unwrap(), "2");
        assert_eq!(query_params.get("types").unwrap(), "custom_type");
        assert_eq!(query_params.get("refs").unwrap(), "reference");
        assert_eq!(query_params.get("addresses").unwrap(), "address");
        assert_eq!(query_params.get("hashes").unwrap(), "hash");
        assert_eq!(query_params.get("channels").unwrap(), "channel");
        assert_eq!(query_params.get("start_date").unwrap(), "0");
        assert_eq!(query_params.get("end_date").unwrap(), "1");
    }
}