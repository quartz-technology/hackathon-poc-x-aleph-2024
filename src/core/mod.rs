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

use crate::mirroring::{Directory, Entry, EntryType, FSTree, File};
use crate::sdk::common::{DefaultEthereumSigner, MessageSigner};
use crate::sdk::post::v0::req_objects::{CreatePostRequest, ListPostsRequest};
use crate::sdk::{self, AlephSDK};
use serde_json;
use tokio::runtime::Runtime;

const TTL: Duration = Duration::from_secs(1);

/// Struct that contains the filesystem inodes and fd handles counter
pub struct FS0X {
    inodes: BTreeMap<Inode, InodeAttributes>,
    file_handles: AtomicU64,
    asdk: AlephSDK,
    fs_tree: FSTree,
    id: String,
}

impl FS0X {
    /// Create a new FS0X filesystem instance
    pub fn new(asdk: AlephSDK, id: String) -> Self {
        Self {
            inodes: BTreeMap::new(),
            file_handles: AtomicU64::new(1),
            asdk: asdk,
            fs_tree: FSTree::new("/".to_string()),
            id: id,
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

    fn sync(&mut self) {
        let signer = DefaultEthereumSigner::new("0xdcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_string()).unwrap();
        let account_address = signer.get_address();

        let params = ListPostsRequest::default()
            .with_channels(vec![format!("fs0x-{}-{}", account_address, self.id)]);

        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(self.asdk.post().v0().list::<String>(params));

        match res {
            Ok(data) => {
                println!("FS TREE FETCHED: {:?}\n", data);

                if let Some(post) = data.posts.first() {
                    let post_content = post.content.clone();
                    let fs_tree: FSTree = serde_json::from_str(&post_content).unwrap();
                    self.fs_tree = fs_tree;

                    for entry in self.fs_tree.entries.iter() {
                        let mut found = false;
                        for inode in self.inodes.values() {
                            if self.get_full_path(inode.inode) == entry.path() {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            let new_inode = self.inodes.last_key_value().unwrap().0 + 1;
                            let path = std::path::Path::new(entry.path());

                            let mut parent_path = match path.parent() {
                                Some(parent) => parent.to_str().unwrap().to_string(),
                                None => {
                                    println!("No parent found");
                                    return;
                                }
                            };

                            if !parent_path.ends_with('/') {
                                parent_path.push('/');
                            }

                            println!("TO CHECK :::: PARENT PATH: {:?}", parent_path);

                            let pinode = match self.get_inode_from_path(&parent_path) {
                                Some(d) => d,
                                None => {
                                    println!("No parent found");
                                    return;
                                }
                            };

                            match self.inodes.get_mut(&pinode.inode) {
                                Some(attr) => {
                                    match &mut attr.kind {
                                        FileKind::Directory(entries) => {
                                            entries.push(new_inode);
                                        },
                                        _ => {
                                            println!("No parent found");
                                            return;
                                        }
                                    }
                                },
                                None => {
                                    println!("No parent found");
                                    return;
                                }
                            }

                            match entry.entry_type() {
                                EntryType::File => {
                                    let new_attr = InodeAttributes {
                                        inode: new_inode,
                                        pinode: Some(pinode.inode),
                                        fname: entry.name().to_string(),
                                        open_file_handles: 0,
                                        size: 0,
                                        last_accessed: std::time::SystemTime::now(),
                                        last_modified: std::time::SystemTime::now(),
                                        last_metadata_changed: std::time::SystemTime::now(),
                                        kind: FileKind::File,
                                        mode: u16::from_str_radix(&entry.permission(), 8).unwrap(),
                                        hardlinks: 1,
                                        uid: 0,
                                        gid: 0,
                                        xattrs: Default::default(),
                                    };
                                    println!("NEW FILE: {:?}", entry.name());
                                    self.inodes.insert(new_inode, new_attr);
                                },
                                EntryType::Directory => {
                                    let new_attr = InodeAttributes {
                                        inode: new_inode,
                                        pinode: Some(pinode.inode),
                                        fname: entry.name().to_string(),
                                        open_file_handles: 0,
                                        size: 0,
                                        last_accessed: std::time::SystemTime::now(),
                                        last_modified: std::time::SystemTime::now(),
                                        last_metadata_changed: std::time::SystemTime::now(),
                                        kind: FileKind::Directory(vec![]),
                                        mode: u16::from_str_radix(&entry.permission(), 8).unwrap(),
                                        hardlinks: 1,
                                        uid: 0,
                                        gid: 0,
                                        xattrs: Default::default(),
                                    };
                                    println!("NEW DIR: {:?}", entry.name());
                                    self.inodes.insert(new_inode, new_attr);
                                },
                                _ => {},
                            }
                        }
                    }

                    println!("NEW FS TREE: {:?}\n", self.fs_tree);
                }
            },
            Err(e) => {
                println!("FS TREE FETCH ERROR: {:?}", e);
            }
        }
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

    pub fn get_inode_from_path(&self, path: &str) -> Option<InodeAttributes> {
        for inode in self.inodes.values() {
            if self.get_full_path(inode.inode) == path {
                return Some(inode.clone());
            }
        }

        None
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
        self.sync();

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
        self.sync();

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

        let f_full_path = self.get_full_path(new_inode);
        println!("TOUCH FILE: {}", f_full_path);

        // TODO: here API call to create file in the cloud
        // the new file path requested is available in `self.get_full_path(new_inode)`
        let new_file = File::new(new_attr.fname.clone(), f_full_path, 0, format!("{:o}", new_attr.mode), new_attr.last_modified, new_attr.gid, new_attr.uid);
        self.fs_tree.add_entry(Entry::File(new_file));

        let fs_tree_json = serde_json::to_string(&self.fs_tree).unwrap();
        println!("FS_TREE: {}", fs_tree_json);

        let signer = DefaultEthereumSigner::new("0xdcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_string()).unwrap();
        let account_address = signer.get_address();

        let params = CreatePostRequest {
            signer: signer,
            channel: format!("fs0x-{}-{}", account_address, self.id),
            custom_type: "fs_tree".to_string(),
            item_type: sdk::common::ItemType::Inline,
            content: fs_tree_json.clone(),
        };
        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(self.asdk.post().v0().create(&params));

        match res {
            Ok(_) => {
                println!("FS_TREE POSTED");
            },
            Err(e) => {
                println!("FS_TREE POST ERROR: {:?}", e);
            }
            
        }

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

        let f_full_path = self.get_full_path(new_inode);
        println!("TOUCH FOLDER: {}", f_full_path);

        // TODO: here API call to create file in the cloud
        // the new file path requested is available in `self.get_full_path(new_inode)`


        let new_dir = Directory::new(new_attr.fname.clone(), f_full_path, format!("{:o}", new_attr.mode), new_attr.last_modified, new_attr.gid, new_attr.uid);
        self.fs_tree.add_entry(Entry::Directory(new_dir));

        let fs_tree_json = serde_json::to_string(&self.fs_tree).unwrap();
        println!("FS_TREE: {}", fs_tree_json);

        let signer = DefaultEthereumSigner::new("0xdcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_string()).unwrap();
        let account_address = signer.get_address();

        let params = CreatePostRequest {
            signer: signer,
            channel: format!("fs0x-{}-{}", account_address, self.id),
            custom_type: "fs_tree".to_string(),
            item_type: sdk::common::ItemType::Inline,
            content: fs_tree_json.clone(),
        };
        let mut rt = Runtime::new().unwrap();
        let res = rt.block_on(self.asdk.post().v0().create(&params));

        match res {
            Ok(_) => {
                println!("FS_TREE POSTED");
            },
            Err(e) => {
                println!("FS_TREE POST ERROR: {:?}", e);
            }
            
        }

        reply.entry(&TTL, &(&new_attr).into(), 0);
    }
}