#![allow(dead_code)]

use std::path::{Path, PathBuf};

use dashtui::docset::Docset;
use rusqlite::Connection;
use tempfile::TempDir;

pub enum FixtureDatabase {
    EmptyPath,
    EmptyTable,
    NoSearchIndex,
    NullPath,
    Valid,
}

impl FixtureDatabase {
    fn sql(&self) -> &'static str {
        match self {
            FixtureDatabase::EmptyPath => {
                include_str!("../fixtures/search_index/empty_path.sql")
            }
            FixtureDatabase::EmptyTable => {
                include_str!("../fixtures/search_index/empty_table.sql")
            }
            FixtureDatabase::NoSearchIndex => {
                include_str!("../fixtures/search_index/no_search_index.sql")
            }
            FixtureDatabase::NullPath => {
                include_str!("../fixtures/search_index/null_path.sql")
            }
            FixtureDatabase::Valid => {
                include_str!("../fixtures/search_index/valid.sql")
            }
        }
    }

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
    pub fn build(&self) -> (TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("docSet.dsidx");
        self.populate(&path);
        (dir, path)
    }

    pub fn build_for_docset(&self, docset: &Docset) {
        self.populate(&docset.index_path());
    }
}

pub fn fixture_docset(index: FixtureDatabase) -> (TempDir, Docset) {
    let dir = tempfile::tempdir().unwrap();
    let docset = Docset {
        path: dir.path().to_path_buf(),
        meta: None,
    };
    std::fs::create_dir_all(docset.index_path().parent().unwrap())
        .expect("Failed to create index dir");
    index.build_for_docset(&docset);
    (dir, docset)
}
