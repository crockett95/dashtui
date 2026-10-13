#![allow(dead_code)]

use std::path::{Path, PathBuf};

use dashtui::docset::Docset;
use rusqlite::Connection;
use tempfile::TempDir;

pub trait FixtureDatabase {
    fn sql(&self) -> &'static str;

    fn populate(&self, path: &Path) {
        let conn = Connection::open(path).unwrap();
        let sql = self.sql();
        conn.execute_batch(sql).unwrap();
    }

    /// Builds a real SQLite file from a fixture's SQL, inside a fresh temp dir.
    ///
    /// Returns the `TempDir` along with the file's path because the directory
    /// is deleted when the `TempDir` is dropped: callers must keep it alive for
    /// the whole test with `let (_dir, path) = ...`. Writing `let (_, path)`
    /// instead would drop it, and delete the file, immediately.
    fn build(&self) -> (TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("docSet.dsidx");
        self.populate(&path);
        (dir, path)
    }

    fn build_for_docset(&self, docset: &Docset) {
        self.populate(&docset.index_path());
    }
}

pub enum SearchIndexFixtureDb {
    EmptyPath,
    EmptyTable,
    Anchors,
    NoSearchIndex,
    NullPath,
    Valid,
}

impl FixtureDatabase for SearchIndexFixtureDb {
    fn sql(&self) -> &'static str {
        match self {
            Self::EmptyPath => {
                include_str!("../fixtures/search_index/empty_path.sql")
            }
            Self::EmptyTable => {
                include_str!("../fixtures/search_index/empty_table.sql")
            }
            Self::Anchors => {
                include_str!("../fixtures/search_index/anchors.sql")
            }
            Self::NoSearchIndex => {
                include_str!("../fixtures/search_index/no_search_index.sql")
            }
            Self::NullPath => {
                include_str!("../fixtures/search_index/null_path.sql")
            }
            Self::Valid => {
                include_str!("../fixtures/search_index/valid.sql")
            }
        }
    }
}

pub enum CoreDataFixtureDb {
    DuplicateMeta,
    EmptyName,
    EmptyPath,
    EmptyTable,
    MissingFile,
    MissingMeta,
    MissingTables,
    MissingType,
    NullAnchor,
    Valid,
}

impl FixtureDatabase for CoreDataFixtureDb {
    fn sql(&self) -> &'static str {
        match self {
            Self::DuplicateMeta => {
                include_str!("../fixtures/core_data/duplicate_metainformation.sql")
            }
            Self::EmptyName => include_str!("../fixtures/core_data/empty_name.sql"),
            Self::EmptyPath => include_str!("../fixtures/core_data/empty_path.sql"),
            Self::EmptyTable => include_str!("../fixtures/core_data/empty_table.sql"),
            Self::MissingFile => include_str!("../fixtures/core_data/missing_file.sql"),
            Self::MissingMeta => include_str!("../fixtures/core_data/missing_metainformation.sql"),
            Self::MissingTables => include_str!("../fixtures/core_data/missing_tables.sql"),
            Self::MissingType => include_str!("../fixtures/core_data/missing_type.sql"),
            Self::NullAnchor => include_str!("../fixtures/core_data/null_anchor.sql"),
            Self::Valid => include_str!("../fixtures/core_data/valid.sql"),
        }
    }
}

pub fn fixture_docset(index: impl FixtureDatabase) -> (TempDir, Docset) {
    let dir = tempfile::tempdir().unwrap();
    let docset = Docset::from((dir.path().to_path_buf(), None));
    std::fs::create_dir_all(docset.index_path().parent().unwrap())
        .expect("Failed to create index dir");
    index.build_for_docset(&docset);
    (dir, docset)
}
