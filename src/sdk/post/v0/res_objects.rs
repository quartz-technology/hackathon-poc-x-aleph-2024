use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::sdk::common::{ItemType, MessageChain};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostMessage<T: Debug> {
    /// The channel where the message was received.
    pub channel: String,
    /// The time the message was received.
    pub time: f64,

    /// The sender's address chain.
    pub chain: MessageChain,
    /// The sender's public address.
    pub sender: String,

    /// The message's hash.
    pub item_hash: String,
    /// The message's storage type.
    pub item_type: ItemType,
    /// The message's content.
    pub content: T,

    /// The message's size.
    pub size: u64,
    /// Was the message confirmed?
    pub confirmed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListPostsResponse<T: Debug> {
    /// The list of posts.
    pub posts: Vec<PostMessage<T>>,
}
