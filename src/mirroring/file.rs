use serde::{Deserialize, Serialize};

use super::{entry::Entry, EntryType};

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub content: String,
    pub permission: u32,
    pub entry_type: EntryType,
}

impl File {
    pub fn new(name: String, path: String, size: u64, permission: u32, content: String) -> Self {
        File {
            name,
            path,
            size,
            permission,
            content,
            entry_type: EntryType::File,
        }
    }
}

impl Entry for File {
    fn path(&self) -> &str {
        &self.path
    }

    fn entry_type(&self) -> EntryType {
        EntryType::File
    }

    fn set_path(&mut self, path: String) {
        self.path = path;
    }
}