use serde::{Deserialize, Serialize};

use super::{directory::Directory, entry::Entry, file::File, sym_link::SymLink};

/// A File System Tree abstraction.
///
/// It stores the root path of the host file system and a list of entries with
/// essential metadatas to mirror the host tree.
///
/// The entries are stored as a list of `Entry` which can be either a `File`,
/// `Directory` or `Symlink`.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FSTree {
    root: &'static str,

    entries: Vec<Entry>,
}

/// The `FSTree` implementation.
/// 
/// When reconstructing the file system tree, the root path is used to locate
/// the original path in the host.  
/// The recommanding way to proceed is to first call `get_dirs` to create the 
/// tree and then `get_files` to load the files' content sequentially and
/// avoid hitting memory limit, the last part is `get_symlinks` to add missing
/// symbolink links.
///
/// This implementation is not optimal and should be improved for large amount
/// of files.
impl FSTree {
    /// Creates a new `FSTree`.
    pub fn new(root: &'static str) -> Self {
        FSTree {
            root,
            entries: Vec::new(),
        }
    }

    pub fn truncate_root(&self, path: &str) -> String {
        if path.starts_with(self.root) {
            return path.strip_prefix(self.root).unwrap().to_string();
        }

        // If the path does not start with the root, return the path as is with the
        // root prefix.
        format!("/{path}")
    }

    /// Adds a new entry to the `FSTree`.
    pub fn add_entry(&mut self, mut entry: Entry) {
        self.entries.push(entry);
    }

    /// Returns the list of entries in the `FSTree`.
    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    /// Returns the list of directories in the `FSTree`.
    pub fn get_dirs(&self) -> Vec<&Directory> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Directory(dir) => Some(dir),
                _ => None,
            })
            .collect()
    }

    /// Returns the list of files in the `FSTree`.
    pub fn get_files(&self) -> Vec<&File> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::File(file) => Some(file),
                _ => None,
            })
            .collect()
    }

    /// Returns the list of symlinks in the `FSTree`.
    pub fn get_symlinks(&self) -> Vec<&SymLink> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::SymLink(symlink) => Some(symlink),
                _ => None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{self},
        os::unix::fs::{MetadataExt, PermissionsExt},
    };

    const TEST_DATA_DIR: &str = "./src/mirroring/testdata";

    /// Mock the implementation of a file system reader.
    fn get_tree(fs: &mut FSTree, path: &str) {
        let test_dir = fs::read_dir(path).unwrap();

        test_dir.for_each(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name();
            let entry_type = entry.file_type().unwrap();
            let permission = format!("{:o}", entry.metadata().unwrap().permissions().mode());
            let created_at = entry.metadata().unwrap().created().unwrap();
            let gid = entry.metadata().unwrap().gid();
            let uid = entry.metadata().unwrap().uid();

            match entry_type {
                _ if entry_type.is_dir() => {
                    let dir = Directory::new(
                        file_name.to_str().unwrap().to_owned(),
                        fs.truncate_root(path.to_str().unwrap()),
                        permission,
                        created_at,
                        gid,
                        uid,
                    );

                    fs.add_entry(Entry::Directory(dir));

                    // Recursively add sub directories and files
                    get_tree(fs,path.to_str().unwrap());
                }
                _ if entry_type.is_file() => {
                    let file = File::new(
                        file_name.to_str().unwrap().to_owned(),
                        fs.truncate_root(path.to_str().unwrap()),
                        entry.metadata().unwrap().size(),
                        permission,
                        created_at,
                        gid,
                        uid,
                    );

                    fs.add_entry(Entry::File(file));
                }
                _ if entry_type.is_symlink() => {
                    let linked_file = fs::read_link(path.clone()).unwrap();

                    let symlink = SymLink::new(
                        file_name.to_str().unwrap().to_owned(),
                        fs.truncate_root(path.to_str().unwrap()),
                        permission,
                        created_at,
                        gid,
                        uid,
                        fs.truncate_root(linked_file.to_str().unwrap()),
                    );

                    fs.add_entry(Entry::SymLink(symlink));
                }
                _ => (),
            }
        });
    }

    /// Compare some fields of two vectors of entries.
    fn compare_entries(expected: &Vec<Entry>, actual: &Vec<Entry>) {
        assert_eq!(
            expected.len(),
            actual.len(),
            "Entries count is not the same"
        );

        for entry in actual {
            let expected_entry = expected
                .as_slice()
                .iter()
                .find(|e| e.path() == entry.path())
                .unwrap();

            assert_eq!(expected_entry.path(), entry.path(), "Path is not the same");
            assert_eq!(
                expected_entry.entry_type(),
                entry.entry_type(),
                "Entry type is not the same"
            );
            assert_eq!(
                expected_entry.permission(),
                entry.permission(),
                "Permission is not the same"
            );
            assert_eq!(expected_entry.name(), entry.name(), "Name is not the same");

            match entry {
                Entry::File(file) => {
                    if let Entry::File(expected_entry) = expected_entry {
                        assert_eq!(
                            expected_entry.size,
                            file.size,
                            "File size is not the same for file: {}",
                            file.path
                        );
                    }
                }
                Entry::SymLink(link) => {
                    if let Entry::SymLink(expected_entry) = expected_entry {
                        assert_eq!(
                            expected_entry.link_to,
                            link.link_to,
                            "Link is not the same for symlink: {}",
                            link.path
                        );
                    }
                }
                _ => (),
            }
        }
    }

    /*
    #[test]
    fn simple_dir_with_one_file() {
        let test_dir_path = format!("{TEST_DATA_DIR}/simple_dir_with_one_file");

        let fs_path = format!("{test_dir_path}/filesystem");
        let mut fs = FSTree::new(fs_path.as_str());

        get_tree(&mut fs, fs_path.as_str());

        let expected_path = format!("{test_dir_path}/expected.json");
        let expected = fs::read_to_string(expected_path).unwrap();
        let expected_fs: FSTree = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected_fs.root, fs.root, "Root path is not the same");
        compare_entries(expected_fs.get_entries(), fs.get_entries());
    }

    #[test]
    fn simple_dir_with_multiple_files() {
        let test_dir_path = format!("{TEST_DATA_DIR}/simple_dir_with_multiple_files");

        let fs_path = format!("{test_dir_path}/filesystem");
        let mut fs = FSTree::new(fs_path.as_str());

        get_tree(&mut fs, fs_path.as_str());

        let expected_path = format!("{test_dir_path}/expected.json");
        let expected = fs::read_to_string(expected_path).unwrap();
        let expected_fs: FSTree = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected_fs.root, fs.root, "Root path is not the same");
        compare_entries(expected_fs.get_entries(), fs.get_entries());
    }

    #[test]
    fn dir_with_one_level_depth() {
        let test_dir_path = format!("{TEST_DATA_DIR}/dir_with_one_level_depth");

        let fs_path = format!("{test_dir_path}/filesystem");
        let mut fs = FSTree::new(fs_path.as_str());

        get_tree(&mut fs, fs_path.as_str());

        let expected_path = format!("{test_dir_path}/expected.json");
        let expected = fs::read_to_string(expected_path).unwrap();
        let expected_fs: FSTree = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected_fs.root, fs.root, "Root path is not the same");
        compare_entries(expected_fs.get_entries(), fs.get_entries());
    }

    #[test]
    fn dir_with_multi_level_depth() {
        let test_dir_path = format!("{TEST_DATA_DIR}/dir_with_multi_level_depth");

        let fs_path = format!("{test_dir_path}/filesystem");
        let mut fs = FSTree::new(fs_path.as_str());

        get_tree(&mut fs, fs_path.as_str());

        let expected_path = format!("{test_dir_path}/expected.json");
        let expected = fs::read_to_string(expected_path).unwrap();
        let expected_fs: FSTree = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected_fs.root, fs.root, "Root path is not the same");
        compare_entries(expected_fs.get_entries(), fs.get_entries());
    }

    #[test]
    fn simple_dir_with_symlink() {
        let test_dir_path = format!("{TEST_DATA_DIR}/simple_dir_with_symlink");

        let fs_path = format!("{test_dir_path}/filesystem");
        let mut fs = FSTree::new(fs_path.as_str());

        get_tree(&mut fs, fs_path.as_str());

        let expected_path = format!("{test_dir_path}/expected.json");
        let expected = fs::read_to_string(expected_path).unwrap();
        let expected_fs: FSTree = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected_fs.root, fs.root, "Root path is not the same");

        compare_entries(expected_fs.get_entries(), fs.get_entries());
    }
     */
}
