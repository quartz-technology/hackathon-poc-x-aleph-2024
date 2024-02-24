use serde::Serialize;
use serde_json;
use sha2::{Digest, Sha256};

use super::{BaseMessage, ItemType};

pub async fn put_content_to_storage_engine<MT, C: Serialize>(message: &mut BaseMessage<C>) {
    let content_json = serde_json::to_string(&message.content).unwrap();

    match content_json.bytes().len() < 50_000 || message.item_type == ItemType::Inline {
        true => {
            message.item_type = ItemType::Inline;
            
            let mut hasher = Sha256::new();
            hasher.update(content_json.as_bytes());

            message.item_hash = format!("{:x}", hasher.finalize());           
            message.item_content = Some(content_json);
        },
        false => {
            message.item_type = ItemType::Storage;
        }
    }
}