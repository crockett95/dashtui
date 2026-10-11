use std::collections::BTreeMap;

use dashtui::docset::{Docset, DocsetError, entry, entry::EntryType, index};
use tempfile::tempdir;

use crate::common::{SearchIndexFixtureDb, fixture_docset};

mod common;
#[test]
fn loads_stats_for_a_valid_index() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::Valid);
    let counts = docset.load_stats().unwrap();
    assert_eq!(
        counts,
        BTreeMap::from([
            (EntryType::Function, 1),
            (EntryType::Builtin, 1),
            (EntryType::Guide, 1),
            (EntryType::Word, 1),
            (EntryType::Variable, 1),
            (EntryType::Parameter, 1),
        ])
    );
}
#[test]
fn returns_empty_stats_if_no_rows() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::EmptyTable);
    let counts = docset.load_stats().unwrap();
    assert_eq!(counts, BTreeMap::new())
}
#[test]
fn returns_error_for_invalid_db() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::NoSearchIndex);
    let counts = docset.load_stats().unwrap_err();
    std::assert_matches!(counts, DocsetError::Index(index::Error::Database(_)))
}
#[test]
fn returns_error_for_invalid_data() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::EmptyPath);
    let counts = docset.load_stats().unwrap_err();
    std::assert_matches!(
        counts,
        DocsetError::Index(index::Error::RowParse {
            id: 20,
            entry_error: entry::Error::EmptyPath
        })
    )
}
#[test]
fn return_error_if_no_index() {
    let dir = tempdir().unwrap();
    let docset = Docset {
        path: dir.path().to_path_buf(),
        meta: None,
    };
    let counts = docset.load_stats().unwrap_err();
    std::assert_matches!(counts, DocsetError::Index(index::Error::Database(_)))
}
