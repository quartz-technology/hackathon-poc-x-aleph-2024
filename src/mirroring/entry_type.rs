use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum EntryType {
    File,
    Directory,
}

impl EntryType {
    fn as_str(&self) -> &'static str {
        match self {
            EntryType::File => "file",
            EntryType::Directory => "directory",
        }
    }
}

impl Serialize for EntryType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for EntryType {
    fn deserialize<D>(deserializer: D) -> Result<EntryType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "file" => Ok(EntryType::File),
            "directory" => Ok(EntryType::Directory),
            _ => Err(serde::de::Error::custom("expected file or directory")),
        }
    }
}