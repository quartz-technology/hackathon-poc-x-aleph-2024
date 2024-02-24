mod inode;

use inode::{Inode, InodeAttributes, FileKind};

use fuser::{
    FileAttr, FileType, Filesystem, MountOption, ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry,
    Request, KernelConfig, ReplyCreate, ReplyWrite,
};

use libc::c_int;
use std::ffi::OsStr;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::atomic::AtomicU64;
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

const TTL: Duration = Duration::from_secs(1);

/// Struct that contains the filesystem inodes and fd handles counter
pub struct FS0X {
    inodes: BTreeMap<Inode, InodeAttributes>,
    file_handles: AtomicU64,
}

impl FS0X {
    /// Create a new FS0X filesystem instance
    pub fn new() -> Self {
        Self {
            inodes: BTreeMap::new(),
            file_handles: AtomicU64::new(1),
        }
    }

    /// Lookup a file or directory by name from a parent directory
    pub fn lookup_name(&self, parent: Inode, name: &OsStr) -> Result<InodeAttributes, c_int> {
        if let Some(attr) = self.inodes.get(&parent) {
            if let FileKind::Directory(entries) = &attr.kind {
                for inode in entries {
                    if let Some(attr) = self.inodes.get(&inode) {
                        if attr.fname == name.to_str().unwrap() {
                            return Ok(attr.clone());
                        }
                    }
                }
            }
        }

        return Err(libc::ENOENT);
    }

    /// Get the full path of a file or directory as an inode
    pub fn get_full_path(&self, inode: Inode) -> String {
        let mut path = String::new();
        let mut current_inode = inode;

        loop {
            match self.inodes.get(&current_inode) {
                Some(attr) => {
                    path = format!("{}/{}", attr.fname, path);
                    match attr.pinode {
                        Some(p) => {
                            current_inode = p;
                        },
                        None => {
                            break;
                        }
                    }
                },
                None => {
                    break;
                }
            }
        }

        path
    }
}

impl Filesystem for FS0X {
    /// Initialize FS0X filesystem
    fn init(&mut self, _req: &Request, _: &mut KernelConfig) -> Result<(), c_int> {
        if !self.inodes.contains_key(&fuser::FUSE_ROOT_ID) {
            let root = InodeAttributes {
                inode: fuser::FUSE_ROOT_ID,
                pinode: None,
                fname: "".to_string(),
                open_file_handles: 0,
                size: 0,
                last_accessed: std::time::SystemTime::now(),
                last_modified: std::time::SystemTime::now(),
                last_metadata_changed: std::time::SystemTime::now(),
                kind: FileKind::Directory(vec![]),
                mode: 0o777,
                hardlinks: 2,
                uid: 0,
                gid: 0,
                xattrs: Default::default(),
            };

            self.inodes.insert(fuser::FUSE_ROOT_ID, root);
        }

        Ok(())
    }

    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if name.to_str() == Some(".") {
            match self.inodes.get(&parent) {
                Some(attr) => {
                    return reply.entry(&TTL, &FileAttr::from(attr), 0);
                },
                None => {
                    return reply.error(libc::ENOENT);
                }
            }
        } else if name.to_str() == Some("..") {
            match self.inodes.get(&parent) {
                Some(attr) => {
                    match attr.pinode {
                        Some(p) => {
                            match self.inodes.get(&p) {
                                Some(pattr) => {
                                    return reply.entry(&TTL, &FileAttr::from(pattr), 0);
                                },
                                None => {
                                    return reply.error(libc::ENOENT);
                                }
                            }
                        },
                        None => {
                            return reply.error(libc::ENOENT);
                        }
                    }
                },
                None => {
                    return reply.error(libc::ENOENT);
                }
            }
        }
        else {
            match self.lookup_name(parent, name) {
                Ok(attr) => {
                    return reply.entry(&TTL, &FileAttr::from(&attr), 0);
                },
                Err(error_code) => {
                    return reply.error(error_code);
                }
            }
        }
    }

    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        match self.inodes.get(&ino) {
            Some(attr) => {
                return reply.attr(&TTL, &FileAttr::from(attr));
            },
            None => {
                return reply.error(libc::ENOENT);
            }
        }
    }

    fn write(
        &mut self,
        _req: &Request,
        inode: u64,
        _fh: u64,
        offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        println!("write() called with {:?} size={:?}", inode, data.len());
        assert!(offset >= 0);

        let _path = self.get_full_path(inode);

        /*if let Ok(mut file) = OpenOptions::new().write(true).open(path) {
            file.seek(SeekFrom::Start(offset as u64)).unwrap();
            file.write_all(data).unwrap();

            let mut attrs = self.get_inode(inode).unwrap();
            attrs.last_metadata_changed = time_now();
            attrs.last_modified = time_now();
            if data.len() + offset as usize > attrs.size as usize {
                attrs.size = (data.len() + offset as usize) as u64;
            }
            clear_suid_sgid(&mut attrs);
            self.write_inode(&attrs);

            reply.written(data.len() as u32);
        } else {
            reply.error(libc::EBADF);
        }*/
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        _size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        match self.inodes.get(&ino) {
            Some(attr) => {
                match &attr.kind {
                    FileKind::File => {
                        // TODO: here API call to read file from the cloud
                        // the file path requested is available in `self.get_full_path(ino)`

                        // tmp dummy return implementation
                        reply.data(&"Hello".as_bytes()[offset as usize..]);
                    },
                    _ => {
                        reply.error(libc::ENOENT);
                    }
                }
            },
            None => {
                reply.error(libc::ENOENT);
            }
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if let Some(attr) = self.inodes.get(&ino) {
            if let FileKind::Directory(entries) = &attr.kind {
                for (i, inode) in entries.iter().enumerate().skip(offset as usize) {
                    if let Some(attr) = self.inodes.get(&inode) {
                        if reply.add(*inode, (i + 1) as i64, attr.kind.clone().into(), &attr.fname) {
                            break;
                        }
                    }
                }
                return reply.ok();
            }
        }

        return reply.error(libc::ENOENT);
    }

    fn create(
        &mut self,
        req: &Request,
        parent: u64,
        name: &OsStr,
        mut mode: u32,
        _umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        if self.lookup_name(parent, name).is_ok() {
            return reply.error(libc::EEXIST);
        }
    
        let new_inode = self.inodes.last_key_value().unwrap().0 + 1;
    
        let new_attr = InodeAttributes {
            inode: new_inode,
            pinode: Some(parent),
            fname: name.to_str().unwrap().to_string(),
            open_file_handles: 0,
            size: 0,
            last_accessed: std::time::SystemTime::now(),
            last_modified: std::time::SystemTime::now(),
            last_metadata_changed: std::time::SystemTime::now(),
            kind: FileKind::File,
            mode: mode as u16,
            hardlinks: 1,
            uid: req.uid(),
            gid: req.gid(),
            xattrs: Default::default(),
        };
    
        match self.inodes.get_mut(&parent) {
            Some(attr) => {
                match &mut attr.kind {
                    FileKind::Directory(entries) => {
                        entries.push(new_inode);
                    },
                    _ => {
                        return reply.error(libc::ENOENT);
                    }
                }
            },
            None => {
                return reply.error(libc::ENOENT);
            }
        }
    
        self.inodes.insert(new_inode, new_attr.clone());
        let fd = self.file_handles.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        println!("TOUCH FILE: {}", self.get_full_path(new_inode));

        // TODO: here API call to create file in the cloud
        // the new file path requested is available in `self.get_full_path(new_inode)`

        reply.created(
            &TTL,
            &(&new_attr).into(),
            0,
            fd,
            0,
        );
    }

    fn mkdir(
        &mut self,
        req: &Request,
        parent: u64,
        name: &OsStr,
        mut mode: u32,
        _umask: u32,
        reply: ReplyEntry,
    ) {
        if self.lookup_name(parent, name).is_ok() {
            return reply.error(libc::EEXIST);
        }

        let mut parent_attrs = match self.inodes.get(&parent) {
            Some(attrs) => attrs.clone(),
            None => {
                return reply.error(libc::EEXIST);
            }
        };

        parent_attrs.last_modified = std::time::SystemTime::now();
        parent_attrs.last_metadata_changed = std::time::SystemTime::now();

        self.inodes.insert(parent, parent_attrs.clone());

        if req.uid() != 0 {
            mode &= !(libc::S_ISUID | libc::S_ISGID) as u32;
        }
        if parent_attrs.mode & libc::S_ISGID as u16 != 0 {
            mode |= libc::S_ISGID as u32;
        }

        let new_inode = self.inodes.last_key_value().unwrap().0 + 1;
    
        let new_attr = InodeAttributes {
            inode: new_inode,
            pinode: Some(parent),
            fname: name.to_str().unwrap().to_string(),
            open_file_handles: 0,
            size: 0,
            last_accessed: std::time::SystemTime::now(),
            last_modified: std::time::SystemTime::now(),
            last_metadata_changed: std::time::SystemTime::now(),
            kind: FileKind::Directory(vec![]),
            mode: mode as u16,
            hardlinks: 1,
            uid: req.uid(),
            gid: req.gid(),
            xattrs: Default::default(),
        };

        match self.inodes.get_mut(&parent) {
            Some(attr) => {
                match &mut attr.kind {
                    FileKind::Directory(entries) => {
                        entries.push(new_inode);
                    },
                    _ => {
                        return reply.error(libc::ENOENT);
                    }
                }
            },
            None => {
                return reply.error(libc::ENOENT);
            }
        }
    
        self.inodes.insert(new_inode, new_attr.clone());

        println!("TOUCH FOLDER: {}", self.get_full_path(new_inode));

        // TODO: here API call to create folder in the cloud
        // the new folder path requested is available in `self.get_full_path(new_inode)`

        reply.entry(&TTL, &(&new_attr).into(), 0);
    }
}