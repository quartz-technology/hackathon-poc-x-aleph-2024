
use serde::Serialize;

use super::{directory::Directory, entry::Entry, entry_type::EntryType, file::File};


#[derive(Debug, Serialize)]
pub struct FileSystem {
    root: String,

    #[serde(with = "serde_entries")]
    entries: Vec<Box<dyn Entry + 'static>>,
}


impl FileSystem {
    pub fn new(root: String) -> Self {
        FileSystem {
            root,
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, mut entry: Box<dyn Entry>) {
        if entry.path().starts_with(self.root.as_str()) {
            entry.set_path(
                entry
                    .path()
                    .strip_prefix(self.root.as_str())
                    .unwrap()
                    .to_string(),
            );
        }

        self.entries.push(entry);
    }

    pub fn get_entries(&self) -> &Vec<Box<dyn Entry>> {
        &self.entries
    }

    pub fn get_dirs(&self) -> Vec<&Directory> {
        let mut dirs = Vec::new();

        for entry in &self.entries {
            if let EntryType::Directory = entry.entry_type() {
                let dir = entry.as_ref().as_any().downcast_ref::<Directory>().unwrap();

                dirs.push(dir)
            }
        }

        dirs
    }

    pub fn get_files(&self) -> Vec<&File> {
        let mut files = Vec::new();

        for entry in &self.entries {
            if let EntryType::File = entry.entry_type() {
                let file = entry.as_ref().as_any().downcast_ref::<File>().unwrap();

                files.push(file)
            }
        }

        files
    }
}

mod serde_entries {
    use super::*;
    use serde::{ser::SerializeSeq, Serializer};

    pub fn serialize<S>(
        entries: &Vec<Box<dyn Entry + 'static>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(entries.len()))?;

        for entry in entries {
            if let EntryType::Directory = entry.entry_type() {
                if let Some(dir) = entry.as_ref().as_any().downcast_ref::<Directory>() {
                    seq.serialize_element(dir)?
                }
            }

            if let EntryType::File = entry.entry_type() {
                if let Some(file) = entry.as_ref().as_any().downcast_ref::<File>() {
                    seq.serialize_element(file)?;
                }
            }
        }

        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, os::unix::fs::PermissionsExt};

    const TEST_DATA_DIR: &str = "./src/mirroring/testdata";

    #[test]
    fn simple_dir_with_one_file() {
        let test_dir_path = format!("{TEST_DATA_DIR}/simple_dir_with_one_file");

        let fs_path = format!("{test_dir_path}/filesystem");
        let expected_path = format!("{test_dir_path}/expected.json");

        let expected = fs::read_to_string(expected_path).unwrap();
        let test_dir = fs::read_dir(fs_path.clone()).unwrap();

        let mut fs = FileSystem::new(fs_path);

        for entry in test_dir {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = entry.file_name();
            let entry_type = entry.file_type().unwrap();

            if entry_type.is_dir() {
                let dir = Directory::new(
                    file_name.to_str().unwrap().to_owned(),
                    path.to_str().unwrap().to_owned(),
                );
                fs.add_entry(Box::new(dir));
            } else {
                let content = fs::read_to_string(path.clone()).unwrap();
                let file = File::new(
                    file_name.to_str().unwrap().to_owned(),
                    path.to_str().unwrap().to_owned(),
                    content.len() as u64,
                    // Fix(TomChv): use the correct permission value
                    entry.metadata().unwrap().permissions().mode(),
                    content,
                );

                fs.add_entry(Box::new(file));
            }
        }

        // TODO(TomChv): finish tests
        let fs_json = serde_json::to_string_pretty(&fs).unwrap();
        println!("{:?}", fs_json);
    }
}
