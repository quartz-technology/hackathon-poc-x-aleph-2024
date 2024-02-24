use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::EntryType;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub permission: String,

    #[serde(with = "serde_millis")]
    pub created_at: SystemTime,
    
    pub gid: u32,
    pub uid: u32,
    pub entry_type: EntryType,
}

impl Directory {
    pub fn new(
        name: String,
        path: String,
        permission: String,
        created_at: SystemTime,
        gid: u32,
        uid: u32,
    ) -> Self {
        Directory {
            name,
            path,
            permission,
            created_at,
            uid,
            gid,
            entry_type: EntryType::Directory,
        }
    }
}
