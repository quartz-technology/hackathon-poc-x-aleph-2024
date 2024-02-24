use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "STORE")]
    Store,    
}

impl MessageType {
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::Post => "POST",
            MessageType::Store => "STORE",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MessageChain {
    #[serde(rename = "ETH")]
    Ethereum,
}

impl MessageChain {
    pub fn as_str(&self) -> &str {
        match self {
            MessageChain::Ethereum => "ETH",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    #[serde(rename = "ipfs")]
    IPFS,
    #[serde(rename = "inline")]
    Inline,
    #[serde(rename = "storage")]
    Storage,
}

impl ItemType {
    pub fn as_str(&self) -> &str {
        match self {
            ItemType::IPFS => "ipfs",
            ItemType::Inline => "inline",
            ItemType::Storage => "storage",
        }
    }
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostContent<C> {
    #[serde(rename = "type")]
    pub custom_type: String,
    pub address: String,
    pub content: C,
    pub time: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMessage<C> {
    /// The channel where the message was received.
    pub channel: String,
    /// The time the message was received.
    pub time: f64,
    /// The message type.
    #[serde(rename = "type")]
    pub message_type: MessageType,

    /// The sender's address chain.
    pub chain: MessageChain,
    /// The sender's public address.
    pub sender: String,

    pub hash_type: String,
    /// The message's hash.
    pub item_hash: String,
    /// The message's storage type.
    pub item_type: ItemType,
    /// The message's content.
    pub content: C,
    /// The string representation of the message's content.
    pub item_content: Option<String>,

    /// The message's signature.
    pub signature: String,
}
