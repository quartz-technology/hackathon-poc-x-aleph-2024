mod req_objects;
mod res_objects;

use std::{collections::HashMap, fmt::Debug, str::FromStr, time::{SystemTime, UNIX_EPOCH}};

use alloy_primitives::Address;
use reqwest::Method;
use serde::Serialize;
use thiserror::Error;
use sha2::{Digest, Sha256};

use crate::{http::{HttpClient, HttpClientError, Request}, sdk::{common::{BaseMessage, ItemType, MessageChain, MessageSigner, MessageSignerError, MessageType, PostContent}, post::v0::res_objects::ListPostsResponse}};
use self::req_objects::{CreatePostRequest, ListPostsRequest};

#[derive(Debug, Error)]
pub enum PostSDKV0Error {
    #[error("http client encountered an error: {0}")]
    Client(#[from] HttpClientError),

    #[error("failed to deserialize response: {0}")]
    ResponseDeserializationError(#[from] reqwest::Error),

    #[error("failed to sign message: {0}")]
    MessageSignError(#[from] MessageSignerError),
}

pub struct PostSDKV0<'a> {
    client: &'a HttpClient,
}

impl <'a>PostSDKV0<'a> {
    pub fn new(client: &'a HttpClient) -> Self {
        PostSDKV0 { client }
    }

    pub async fn list<T: serde::de::DeserializeOwned + Debug>(&self, params: ListPostsRequest) -> Result<ListPostsResponse<T>, PostSDKV0Error> {
        let query_params = params.query_params();
        
        let req = Request {
            method: Method::GET,
            path: "/api/v0/posts.json".to_string(),
            query_params,
        };

        let res = self.client.do_request(req).await?;

        let data = res
            .json::<ListPostsResponse<T>>()
            .await
            .map_err(PostSDKV0Error::ResponseDeserializationError)?;

        Ok(data)
    }

    pub async fn create<T: Serialize + Copy + Debug>(&self, params: &CreatePostRequest<T>) -> Result<(), PostSDKV0Error> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards ???").as_secs();

        let addr = Address::from_str(params.signer.get_address().as_str()).expect("parse l'addresse non ???").to_checksum(None);

        let post_content = PostContent {
            custom_type: params.custom_type.clone(),
            address: addr.clone(),
            content: params.content,
            time: timestamp as f64,
        };

        let mut message = BaseMessage {
            channel: params.channel.clone(),
            time: timestamp as f64,
            message_type: MessageType::Post,
            chain: MessageChain::Ethereum,
            sender: addr.clone(),
            hash_type: "sha256".to_string(),
            item_hash: "".to_string(),
            item_type: params.item_type,
            content: post_content,
            item_content: None,
            signature: "".to_string(),
        };

        let content_json = serde_json::to_string(&message.content).unwrap();

        match content_json.bytes().len() < 50_000 && message.item_type == ItemType::Inline {
            true => {
                message.item_type = ItemType::Inline;
                
                let mut hasher = Sha256::new();
                hasher.update(content_json.as_bytes());

                message.item_hash = format!("{:x}", hasher.finalize());           
                message.item_content = Some(content_json);
            },
            false => {
                message.item_type = ItemType::Storage;

                let req = Request {
                    method: Method::POST,
                    path: "/api/v0/storage/add_json".to_string(),
                    query_params: HashMap::new(),
                };

                let res = self.client.do_post_request(req, content_json).await?;
                let data = res
                    .text()
                    .await
                    .map_err(PostSDKV0Error::ResponseDeserializationError)?;

                println!("Storage response: {}", data);
            }
        }

        message.signature = params.signer.sign(&message).await?;

        let body = PubSubNotifyRequest {
            topic: "ALEPH-TEST".to_string(),
            data: serde_json::to_string(&message).unwrap(),
        };

        let req = Request {
            method: Method::POST,
            path: "/api/v0/ipfs/pubsub/pub".to_string(),
            query_params: HashMap::new(),
        };

        let res = self.client.do_post_request(req, body).await?;

        let data = res
            .text()
            .await
            .map_err(PostSDKV0Error::ResponseDeserializationError)?;

        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct PubSubNotifyRequest {
    pub topic: String,
    pub data: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{http::HttpClient, sdk::common::DefaultEthereumSigner};
    use serde::Deserialize;

    #[derive(Debug, Deserialize, Serialize, Clone, Copy)]
    #[serde(tag = "metrics")]
    struct Metrics {}

    #[tokio::test]
    async fn it_lists_posts() {
        let client = HttpClient::new().unwrap();
        let sdk = PostSDKV0::new(&client);

        let params = ListPostsRequest::default().with_hashes(vec![
            "b33110b8c8e9d8d6dc67813007c5b8318ed3720776c4ee6431cc60ee4b0d18ad".to_string(),
        ]);
        let _posts = sdk.list::<Metrics>(params)
            .await
            .unwrap();
    }

    #[derive(Debug, Serialize, Clone, Copy)]
    struct TestPostMessage {
        pub content: i64,
    }

    #[tokio::test]
    async fn it_creates_post() {
        let client = HttpClient::new().unwrap();
        let sdk = PostSDKV0::new(&client);

        let signer = DefaultEthereumSigner::new("0xdcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_string()).unwrap();
        let params = CreatePostRequest {
            signer,
            custom_type: "fs0x-test".to_string(),
            content: TestPostMessage {
                content: 42,
            },
            channel: "fsx".to_string(),
            item_type: ItemType::Inline,
        };

        sdk.create(&params)
            .await
            .unwrap();
    }
}
