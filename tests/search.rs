use dashtui::docset::{self, Docset, entry::Entry, index};
use tempfile::tempdir;

use crate::common::{CoreDataFixtureDb, FixtureDatabase, SearchIndexFixtureDb, fixture_docset};

mod common;

fn names<'a>(hits: &[&'a Entry]) -> Vec<&'a str> {
    hits.iter().map(|i| i.name()).collect()
}

#[test]
fn finds_entries_in_search_index_docset() {
    // Valid has `Introduction`, `abort (C-g)`, and single-symbol names.
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::Valid);
    let hits = names(&docset.search("intro").unwrap());
    assert_eq!(hits, ["Introduction"])
}

#[test]
fn finds_entries_in_core_data_docset() {
    // Valid has `lua_pushvfstring`, `LUA_LOADLIBK`, and `utf8.codes`.
    let (_dir, docset) = fixture_docset(CoreDataFixtureDb::Valid);
    let hits = names(&docset.search("lua_").unwrap());
    assert_eq!(hits, ["lua_pushvfstring", "LUA_LOADLIBK"])
}

#[test]
fn returns_error_for_invalid_db() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::NoSearchIndex);
    let err = docset.search("intro").unwrap_err();
    std::assert_matches!(
        err,
        docset::Error::Index {
            index_error: index::Error::Database(_),
            path: _
        }
    )
}

#[test]
fn returns_error_for_invalid_data() {
    // EmptyPath's bad row has id 20.
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::EmptyPath);
    let err = docset.search("intro").unwrap_err();
    std::assert_matches!(
        err,
        docset::Error::Index {
            index_error: index::Error::RowParse {
                id: 20,
                entry_error: _
            },
            path: _
        }
    )
}

#[test]
fn returns_error_if_no_index() {
    let dir = tempdir().unwrap();
    let docset = Docset::from((dir.path().to_path_buf(), None));
    let err = docset.search("intro").unwrap_err();
    std::assert_matches!(
        err,
        docset::Error::Index {
            index_error: index::Error::Database(_),
            path: _,
        }
    )
}

#[test]
fn reuses_entries_after_index_is_deleted() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::Valid);
    let first = names(&docset.search("intro").unwrap());
    std::fs::remove_file(docset.index_path()).unwrap();
    let second = names(&docset.search("intro").unwrap());
    assert_eq!(second, first)
}

#[test]
fn search_reuses_entries_loaded_by_stats() {
    let (_dir, docset) = fixture_docset(SearchIndexFixtureDb::Valid);
    docset.load_stats().unwrap();
    std::fs::remove_file(docset.index_path()).unwrap();
    let hits = names(&docset.search("intro").unwrap());
    assert_eq!(hits, ["Introduction"])
}

#[test]
fn retries_load_after_a_failed_one() {
    // A failed load is not cached: once the index appears, the next search reads it.
    let dir = tempdir().unwrap();
    let docset = Docset::from((dir.path().to_path_buf(), None));
    docset.search("intro").unwrap_err();
    std::fs::create_dir_all(docset.index_path().parent().unwrap()).unwrap();
    SearchIndexFixtureDb::Valid.build_for_docset(&docset);
    let second = names(&docset.search("intro").unwrap());
    assert_eq!(second, ["Introduction"])
}
