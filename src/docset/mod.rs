pub mod entry;
pub mod index;
pub mod meta;
use std::collections::BTreeMap;
use std::io;
use std::io::ErrorKind::{NotADirectory, NotFound};
use std::path::{Path, PathBuf};

use nucleo_matcher::pattern::{AtomKind, CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher};
// TODO Use std::OnceCell when get_or_try_init is stable
use once_cell::sync::OnceCell;
use serde::Serialize;

use crate::docset::entry::{Entry, EntryType};
use crate::docset::index::read_entries;
use crate::docset::meta::DocsetMeta;
use crate::stats::count_types;

#[derive(Debug, Serialize)]
pub struct Docset {
    path: PathBuf,
    #[serde(flatten)]
    meta: Option<DocsetMeta>,
    #[serde(skip)]
    entries: OnceCell<Vec<Entry>>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read index {path}")]
    Index {
        #[source]
        index_error: index::Error,
        path: PathBuf,
    },
}

impl Docset {
    /// The docset's metadata from its `Info.plist`.
    ///
    /// [`None`] if the plist was missing or couldn't be parsed when the
    /// `Docset` was created (see [`Docset::load`]).
    pub fn meta(&self) -> Option<&DocsetMeta> {
        self.meta.as_ref()
    }

    /// The docset's root directory, the one whose name ends in `.docset`.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The path to the docset's index database, `docSet.dsidx`.
    ///
    /// The path is built from [`Docset::path`] alone; the file is not checked
    /// for existence.
    pub fn index_path(&self) -> PathBuf {
        self.path.join("Contents/Resources/docSet.dsidx")
    }

    /// Counts the docset's entries by [`EntryType`].
    ///
    /// Types are counted after normalization, so aliases such as `func` and
    /// `Function` are counted together. The map is ordered by type, so output
    /// built from it is deterministic.
    ///
    /// Loads the index on first use and caches its entries; see
    /// [`Docset::search`], which shares the same cache.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Index`] if the index can't be loaded: the file is
    /// missing or isn't a recognized index, or a row can't be read or isn't a
    /// valid entry. A failed load isn't cached, so the next call tries again.
    pub fn load_stats(&self) -> Result<BTreeMap<EntryType, usize>, Error> {
        let index = self.load_index()?;
        Ok(count_types(index))
    }

    /// Finds the entries whose name fuzzy-matches `query`, best match first.
    ///
    /// Matching is fzf-style, done by `nucleo-matcher`: an entry matches if
    /// the query's characters appear in its name in order, not necessarily
    /// next to each other (`rdarr` finds `readarray`). Only an entry's name is
    /// matched, never its path.
    ///
    /// Case is smart: an all-lowercase query ignores case, and a query with
    /// any uppercase letter is case-sensitive (`READ` finds `READLINE_LINE`
    /// but not `read`). Ignoring case covers non-ASCII letters too, so `ärger`
    /// finds `Ärger`.
    ///
    /// Results are ordered by match score, highest first, with tied scores
    /// left in index order. An empty query matches every entry, in index
    /// order. Results borrow from this `Docset`.
    ///
    /// Loads the index on first use and caches its entries, so later calls,
    /// including [`Docset::load_stats`], don't read the file again.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Index`] if the index can't be loaded: the file is
    /// missing or isn't a recognized index, or a row can't be read or isn't a
    /// valid entry. A failed load isn't cached, so the next call tries again.
    pub fn search(&self, query: &str) -> Result<Vec<&Entry>, Error> {
        let entries = self.load_index()?;
        let mut matcher = Matcher::new(Config::DEFAULT);
        let matches = Pattern::new(
            query,
            CaseMatching::Smart,
            Normalization::Smart,
            AtomKind::Fuzzy,
        )
        .match_list(entries, &mut matcher);
        Ok(matches.into_iter().map(|(a, _)| a).collect())
    }

    /// The docset's entries, reading the index on the first successful call.
    ///
    /// On error the cache stays empty, so the next call retries the read.
    fn load_index(&self) -> Result<&[Entry], Error> {
        let entries = self
            .entries
            .get_or_try_init(|| read_entries(&self.index_path()))
            .map_err(|e| Error::Index {
                index_error: e,
                path: self.index_path(),
            })?;
        Ok(entries)
    }

    /// Fills the entry cache directly, so tests can search without an index
    /// file.
    ///
    /// # Panics
    ///
    /// Panics if the cache is already filled, by an earlier load or sideload.
    #[cfg(test)]
    pub(crate) fn sideload_index(&self, entries: Vec<Entry>) {
        self.entries
            .set(entries)
            .unwrap_or_else(|_| panic!("Failed to load entries for {self:?}"));
    }

    /// Creates a `Docset` from the path to its `.docset` directory.
    ///
    /// Reads the docset's metadata from its `Info.plist` with
    /// [`DocsetMeta::load`]. Missing or invalid metadata doesn't fail: the
    /// `Docset` is still created, and [`Docset::meta`] returns [`None`].
    ///
    /// The index isn't read here; [`Docset::search`] and
    /// [`Docset::load_stats`] load it on first use.
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
        Self {
            path,
            meta,
            entries: OnceCell::new(),
        }
    }
}

// Testing method for integration tests. Not for public consumption
#[doc(hidden)]
impl From<(PathBuf, Option<DocsetMeta>)> for Docset {
    fn from((path, meta): (PathBuf, Option<DocsetMeta>)) -> Self {
        Self {
            path,
            meta,
            entries: OnceCell::new(),
        }
    }
}

/// Searches `root` for docset directories: any directory whose name ends in
/// `.docset`.
///
/// The search is recursive, but does **not** look inside a directory once
/// it's been identified as a docset — a `.docset` directory's own contents
/// (its index, its HTML) are not themselves searched for nested
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
                entries: OnceCell::new(),
            };
            let index_path = PathBuf::from(index);

            assert_eq!(docset.index_path(), index_path, "case {d}: {index}")
        }
    }

    mod search_index {
        use super::*;
        fn names<'a>(hits: &[&'a Entry]) -> Vec<&'a str> {
            hits.iter().map(|i| i.name()).collect()
        }

        fn entry(name: &str, path: &str) -> Entry {
            Entry::try_from((name, "Function", path, None)).unwrap()
        }

        /// A docset with entries already loaded, so no index file is needed.
        ///
        /// Names are mostly real Bash.docset entries, chosen so one query can
        /// hit several of them in different cases (`read`, `READLINE_LINE`,
        /// `readarray`). `printf` has `read` in its path but not its name.
        /// `Ärger` is non-ASCII, `abort (C-g)` has spaces and punctuation, and
        /// `:`, `!`, `-` are real single-symbol names.
        fn docset() -> Docset {
            let docset = Docset::from((PathBuf::from("Test.docset"), None));
            docset.sideload_index(vec![
                entry("read", "bash/Bash-Builtins.html#read"),
                entry("READLINE_LINE", "bash/Bash-Variables.html#READLINE_LINE"),
                entry("readarray", "bash/Bash-Builtins.html#readarray"),
                entry("printf", "bash/read-adjacent/printf.html"),
                entry("abort (C-g)", "bash/Miscellaneous-Commands.html#abort"),
                entry("Ärger", "bash/umlaut.html"),
                entry(":", "bash/Bourne-Shell-Builtins.html#colon"),
                entry("!", "bash/Pipelines.html#bang"),
                entry("-", "bash/Special-Parameters.html#dash"),
            ]);
            docset
        }

        #[test]
        fn matches_lowercase_query_against_any_case() {
            let docset = docset();
            let hits = names(&docset.search("read").unwrap());
            let expected = ["read", "READLINE_LINE", "readarray"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn matches_uppercase_query_against_uppercase() {
            let docset = docset();
            let hits = names(&docset.search("READ").unwrap());
            let expected = ["READLINE_LINE"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn matches_substring_in_middle_of_name() {
            let docset = docset();
            let hits = names(&docset.search("LINE").unwrap());
            let expected = ["READLINE_LINE"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn fuzzy_matches_entry_name() {
            let docset = docset();
            let hits = names(&docset.search("rdarr").unwrap());
            let expected = ["readarray"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn returns_empty_when_nothing_matches() {
            let docset = docset();
            let hits = docset.search("zzz").unwrap();
            assert!(hits.is_empty());
        }

        #[test]
        fn empty_query_returns_every_entry() {
            let docset = docset();
            let hits = names(&docset.search("").unwrap());
            let expected = [
                "read",
                "READLINE_LINE",
                "readarray",
                "printf",
                "abort (C-g)",
                "Ärger",
                ":",
                "!",
                "-",
            ];
            assert_eq!(hits, expected)
        }

        #[test]
        fn matches_name_not_path() {
            let docset = docset();
            let hits = docset.search("adjacent").unwrap();
            assert!(hits.is_empty());
        }

        #[test]
        fn matches_non_ascii_case_insensitively() {
            let docset = docset();
            let hits = names(&docset.search("ärger").unwrap());
            let expected = ["Ärger"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn matches_punctuation_and_spaces() {
            let docset = docset();
            let hits = names(&docset.search("(c-g)").unwrap());
            let expected = ["abort (C-g)"];
            assert_eq!(hits, expected)
        }

        #[test]
        fn matches_single_symbol_name() {
            let docset = docset();
            let hits: Vec<&str> = names(&docset.search(":").unwrap());
            let expected = [":"];
            assert_eq!(hits, expected)
        }
    }
}
