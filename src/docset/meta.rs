use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DocsetMeta {
    #[serde(rename(deserialize = "CFBundleIdentifier"))]
    pub id: String,
    #[serde(rename(deserialize = "CFBundleName"))]
    pub name: String,
    #[serde(rename(deserialize = "DocSetPlatformFamily"))]
    pub platform_family: String,
    #[serde(rename(deserialize = "dashIndexFilePath"))]
    pub index_file: Option<PathBuf>,
}

#[derive(thiserror::Error, Debug)]
pub enum DocsetMetaError {
    #[error("Couldn't read docset: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to parse plist: {0}")]
    Plist(#[from] plist::Error),
}

impl DocsetMeta {
    pub fn load(docset_dir: &Path) -> Result<Self, DocsetMetaError> {
        let plist_path = docset_dir.join("Contents/Info.plist");
        let file = File::open(plist_path)?;
        let meta: DocsetMeta = plist::from_reader(file)?;
        Ok(meta)
    }
}
