use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::EntryType;

/// A symbolic link in the mirroring.
///
/// It stores UNIX-like file system information about a symbolic link.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct SymLink {
    pub name: String,

    /// The absolute path of the directory.
    pub path: String,

    /// Permisions are stored as octal strings value (e.g "100644" -> "0o644").
    ///
    /// Use `u32::from_str_radix(&permission, 8).unwrap()` to convert it to a `u32`.
    pub permission: String,

    #[serde(with = "serde_millis")]
    pub created_at: SystemTime,

    /// The group id of the directory.
    pub gid: u32,

    /// The user id of the directory.
    pub uid: u32,

    #[serde(rename = "type")]
    pub entry_type: EntryType,

    /// The path this symbolic link is linked to.
    pub link_to: String
}

impl SymLink {
    /// Creates a new `SymLink`.
    pub fn new(
        name: String,
        path: String,
        permission: String,
        created_at: SystemTime,
        gid: u32,
        uid: u32,
        link_to: String
    ) -> Self {
        SymLink {
            name,
            path,
            permission,
            created_at,
            uid,
            gid,
            entry_type: EntryType::SymLink,
            link_to
        }
    }
}
