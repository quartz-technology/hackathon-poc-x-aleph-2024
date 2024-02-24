use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use std::time::SystemTime;

const BLOCK_SIZE: u64 = 512;

pub type Inode = u64;

#[derive(Serialize, Deserialize, Clone)]
pub struct InodeAttributes {
    pub inode: Inode,
    pub pinode: Option<Inode>,
    pub fname: String,
    pub open_file_handles: u64,
    pub size: u64,
    pub last_accessed: SystemTime,
    pub last_modified: SystemTime,
    pub last_metadata_changed: SystemTime,
    pub kind: FileKind,
    pub mode: u16,
    pub hardlinks: u32,
    pub uid: u32,
    pub gid: u32,
    pub xattrs: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl From<&InodeAttributes> for fuser::FileAttr {
    fn from(attrs: &InodeAttributes) -> Self {
        fuser::FileAttr {
            ino: attrs.inode,
            size: attrs.size,
            blocks: (attrs.size + BLOCK_SIZE - 1) / BLOCK_SIZE,
            atime: attrs.last_accessed,
            mtime: attrs.last_modified,
            ctime: attrs.last_metadata_changed,
            crtime: SystemTime::UNIX_EPOCH,
            kind: attrs.kind.clone().into(),
            perm: attrs.mode,
            nlink: attrs.hardlinks,
            uid: attrs.uid,
            gid: attrs.gid,
            rdev: 0,
            blksize: BLOCK_SIZE as u32,
            flags: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum FileKind {
    File,
    Directory(Vec<Inode>),
}

impl From<FileKind> for fuser::FileType {
    fn from(kind: FileKind) -> Self {
        match kind {
            FileKind::File => fuser::FileType::RegularFile,
            FileKind::Directory(_) => fuser::FileType::Directory,
        }
    }
}