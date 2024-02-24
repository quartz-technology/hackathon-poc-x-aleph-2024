use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EntryType {
    #[serde(rename = "file")]   
    File,

    #[serde(rename = "directory")]
    Directory,
}