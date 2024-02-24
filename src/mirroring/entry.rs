use serde::{Deserialize, Serialize};

use super::{Directory, EntryType, File};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Entry {
    File(File),
    Directory(Directory),
}

impl Entry {
    pub fn path(&self) -> &str {
        match self {
            Entry::File(file) => &file.path,
            Entry::Directory(dir) => &dir.path,
        }
    }

    pub fn set_path(&mut self, path: String) {
        match self {
            Entry::File(file) => file.path = path,
            Entry::Directory(dir) => dir.path = path,
        }
    }

    pub fn entry_type(&self) -> &EntryType {
        match self {
            Entry::File(file) => &file.entry_type,
            Entry::Directory(dir) => &dir.entry_type,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Entry::File(file) => &file.name,
            Entry::Directory(dir) => &dir.name,
        }
    }

    pub fn permission(&self) -> &str {
        match self {
            Entry::File(file) => &file.permission,
            Entry::Directory(dir) => &dir.permission,
        }
    }
}
