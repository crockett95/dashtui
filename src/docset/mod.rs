pub mod entry;
pub mod meta;
use std::io;
use std::io::ErrorKind::{NotADirectory, NotFound};
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::docset::meta::DocsetMeta;

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct Docset {
    pub path: PathBuf,
    #[serde(flatten)]
    pub meta: Option<DocsetMeta>,
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
                    let meta = match DocsetMeta::load(&child.path()) {
                        Ok(meta) => Some(meta),
                        Err(err) => {
                            eprintln!(
                                "Error reading metadata for {}: {}",
                                child.path().to_string_lossy(),
                                err
                            );
                            None
                        }
                    };
                    results.push(Docset {
                        path: child.path(),
                        meta,
                    });
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
