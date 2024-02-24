use serde::{Deserialize, Serialize};

use super::{entry::Entry, EntryType};

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub path: String,
    pub entry_type: EntryType,
}

impl Entry for Directory {
    fn path(&self) -> &str {
        &self.path
    }

    fn entry_type(&self) -> EntryType {
        EntryType::Directory
    }

    fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

impl Directory {
    pub fn new(name: String, path: String) -> Self {
        Directory {
            name,
            path,
            entry_type: EntryType::Directory,
        }
    }
}