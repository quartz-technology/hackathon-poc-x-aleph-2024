use serde::{Deserialize, Serialize};

/// The type of entry supported in the mirroring.
///
/// Add a new entry type here if you want to support a new type of entry in
/// the mirroring.
///
/// The type should also be added to the `Entry` enum in the `entry.rs` file.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EntryType {
    #[serde(rename = "file")]
    File,

    #[serde(rename = "directory")]
    Directory,

    #[serde(rename = "symlink")]
    SymLink,
}
