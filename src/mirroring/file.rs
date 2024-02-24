use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::EntryType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub permission: String,

    #[serde(with = "serde_millis")]
    pub created_at: SystemTime,
    
    pub gid: u32,
    pub uid: u32,
    pub entry_type: EntryType,
}

impl File {
    pub fn new(
        name: String,
        path: String,
        size: u64,
        permission: String,
        created_at: SystemTime,
        gid: u32,
        uid: u32,
    ) -> Self {
        File {
            name,
            path,
            size,
            permission,
            created_at,
            uid,
            gid,
            entry_type: EntryType::File,
        }
    }
}
