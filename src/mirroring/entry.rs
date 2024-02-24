use serde::{Deserialize, Serialize};

use super::{Directory, EntryType, File};

/// An entry in the mirroring.
///
/// Add a new entry type here if you want to support a new type of entry in
/// the mirroring.
///
/// The type should also be added to the `EntryType` enum in the `entry_type.rs` 
/// file.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Entry {
    File(File),
    Directory(Directory),
}

/// Implementations for the `Entry` enum to get accessor and setter on some
/// generic fields.
impl Entry {
    /// Returns the path of the entry.
    pub fn path(&self) -> &str {
        match self {
            Entry::File(file) => &file.path,
            Entry::Directory(dir) => &dir.path,
        }
    }

    /// Sets the path of the entry.
    pub fn set_path(&mut self, path: String) {
        match self {
            Entry::File(file) => file.path = path,
            Entry::Directory(dir) => dir.path = path,
        }
    }

    /// Returns the type of the entry.
    pub fn entry_type(&self) -> &EntryType {
        match self {
            Entry::File(file) => &file.entry_type,
            Entry::Directory(dir) => &dir.entry_type,
        }
    }

    /// Returns the name of the entry.
    pub fn name(&self) -> &str {
        match self {
            Entry::File(file) => &file.name,
            Entry::Directory(dir) => &dir.name,
        }
    }

    /// Returns the permission of the entry.
    pub fn permission(&self) -> &str {
        match self {
            Entry::File(file) => &file.permission,
            Entry::Directory(dir) => &dir.permission,
        }
    }
}
