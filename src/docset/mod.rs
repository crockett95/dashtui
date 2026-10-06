pub mod entry;
pub mod index;
pub mod meta;
use std::collections::BTreeMap;
use std::io;
use std::io::ErrorKind::{NotADirectory, NotFound};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::docset::entry::EntryType;
use crate::docset::index::search_index::{IndexError, read_entries};
use crate::docset::meta::DocsetMeta;
use crate::stats::count_types;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Docset {
    pub path: PathBuf,
    #[serde(flatten)]
    pub meta: Option<DocsetMeta>,
}

#[derive(Debug, thiserror::Error)]
pub enum DocsetError {
    #[error("Failed to read index")]
    Index(#[from] IndexError),
}

impl Docset {
    /// Returns the path to the docset's index DB.
    pub fn index_path(&self) -> PathBuf {
        self.path.join("Contents/Resources/docSet.dsidx")
    }

    /// Get the stats for a docset
    pub fn load_stats(&self) -> Result<BTreeMap<EntryType, usize>, DocsetError> {
        let index = read_entries(&self.index_path())?;
        Ok(count_types(&index))
    }

    /// Creates a `Docset` from a [`PathBuf`]
    ///
    /// Given a path to a docset, constructs the `Docset` including reading its
    /// metadata from the Info.plist if it exists and is valid according to
    /// [`DocsetMeta::load`]. Invalid or nonexistant metadata results in
    /// [`Docset::meta`] being [`None`]
    pub fn load(path: PathBuf) -> Self {
        let meta = match DocsetMeta::load(&path) {
            Ok(meta) => Some(meta),
            Err(err) => {
                // TODO: Replace with real logging
                eprintln!(
                    "Error reading metadata for {}: {}",
                    path.to_string_lossy(),
                    err
                );
                None
            }
        };
        Self { path, meta }
    }
}

/// Searches `root` for docset directories: any directory whose name ends in
/// `.docset`.
///
/// The search is recursive, but does **not** look inside a directory once
/// it's been identified as a docset — a `.docset` directory's own contents
/// (its search index, its HTML) are not themselves searched for nested
/// docsets.
///
/// Returns every match found, in no particular order.
///
/// # Errors
///
/// Returns an error if `root` itself cannot be read (it doesn't exist, isn't
/// a directory, or its contents can't be listed for another reason, e.g.
/// permissions). Errors reading a *subdirectory* encountered during the walk
/// are reported to stderr which is clunky but I don't know how to do better yet.
pub fn find_docsets(root: &Path) -> io::Result<Vec<Docset>> {
    if !root.exists() {
        return Err(io::Error::new(
            NotFound,
            format!("Path {} does not exist", root.display()),
        ));
    } else if !root.is_dir() {
        return Err(io::Error::new(
            NotADirectory,
            format!("Path {} is not a directory", root.display()),
        ));
    }
    let mut results = Vec::new();

    for child in root
        .read_dir()
        .unwrap_or_else(|_| panic!("Unable to read {}", root.display()))
    {
        match child {
            Ok(child) if child.file_type().unwrap().is_dir() => {
                if child.path().extension().is_some_and(|ext| ext == "docset") {
                    results.push(Docset::load(child.path()));
                } else {
                    results.extend(find_docsets(&child.path())?);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
            _ => (),
        }
    }
    Ok(results)
}

// See tests/discovery.rs for the tests against tests/fixtures/docsets_root/.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn builds_index_path_from_docset_path() {
        for (d, index) in [
            ("/tmp/foo", "/tmp/foo/Contents/Resources/docSet.dsidx"),
            (
                "docsets/Bash.docset",
                "docsets/Bash.docset/Contents/Resources/docSet.dsidx",
            ),
        ] {
            let docset = Docset {
                path: PathBuf::from(d),
                meta: None,
            };
            let index_path = PathBuf::from(index);

            assert_eq!(docset.index_path(), index_path, "case {d}: {index}")
        }
    }
}
