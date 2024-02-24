use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::EntryType;

/// A file in the mirroring.
///
/// It stores UNIX-like file system information about a file.
///
/// This struct does not store the content of the file.
/// It assumes the caller library has an access to the content using the path.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct File {
    pub name: String,
    pub size: u64,

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
}

impl File {
    /// Creates a new `File`.
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
