use serde::{Deserialize, Serialize};

use super::{directory::Directory, entry::Entry, file::File};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FSTree<'a> {
    root: &'a str,

    entries: Vec<Entry>,
}

impl<'a> FSTree<'a> {
    pub fn new(root: &'a str) -> Self {
        FSTree {
            root,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, mut entry: Entry) {
        if entry.path().starts_with(self.root) {
            entry.set_path(entry.path().strip_prefix(self.root).unwrap().to_string());
        }

        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_dirs(&self) -> Vec<&Directory> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::Directory(dir) => Some(dir),
                _ => None,
            })
            .collect()
    }

    pub fn get_files(&self) -> Vec<&File> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                Entry::File(file) => Some(file),
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
                        path.to_str().unwrap().to_owned(),
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
                        path.to_str().unwrap().to_owned(),
                        entry.metadata().unwrap().size(),
                        permission,
                        created_at,
                        gid,
                        uid,
                    );

                    fs.add_entry(Entry::File(file));
                }
                _ => (),
            }
        });
    }

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
        }
    }

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
}