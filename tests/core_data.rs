mod common;
use dashtui::docset::{
    entry::{self, Entry, RawEntry},
    index::{self, read_entries},
};

use crate::common::{CoreDataFixtureDb, FixtureDatabase};

/// `core_data/valid.sql`: three real Lua rows come back in token-id order, with
/// `func` normalized to `Function` and each row's own anchor.
#[test]
fn reads_every_row_of_a_valid_index() {
    let (_dir, path) = CoreDataFixtureDb::Valid.build();
    let entries = read_entries(&path).unwrap();

    let expected = [
        (
            "lua_pushvfstring",
            "Function",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/lua%5Fpushvfstring"),
        ),
        (
            "LUA_LOADLIBK",
            "Function",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/LUA%5FLOADLIBK"),
        ),
        (
            "utf8.codes",
            "Function",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/utf8%2Ecodes"),
        ),
    ]
    .map(|(name, kind, path, anchor)| RawEntry {
        name,
        kind,
        path,
        anchor,
    })
    .map(|row| Entry::try_from(row).unwrap());
    assert_eq!(entries, expected);
}

/// `core_data/duplicate_metainformation.sql`: a token with two metadata rows
/// yields one entry per row, like any symbol documented in two places.
#[test]
fn returns_all_entries_if_duplicate_meta() {
    let (_dir, path) = CoreDataFixtureDb::DuplicateMeta.build();
    let entries = read_entries(&path).unwrap();
    let expected = [
        (
            "print",
            "func",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/print"),
        ),
        (
            "pairs",
            "func",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/pairs"),
        ),
        (
            "pairs",
            "func",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/pairs2"),
        ),
        (
            "type",
            "func",
            "www.lua.org/manual/5.5/manual.html",
            Some("//apple_ref/func/type"),
        ),
    ]
    .map(|(name, kind, path, anchor)| RawEntry {
        name,
        kind,
        path,
        anchor,
    })
    .map(|row| Entry::try_from(row).unwrap());
    assert_eq!(entries, expected);
}

/// `core_data/empty_name.sql`: an empty `ZTOKENNAME` is a parse error that
/// carries the token's id (20, not its position).
#[test]
fn errors_on_empty_names() {
    let (_dir, path) = CoreDataFixtureDb::EmptyName.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(
        error,
        index::Error::RowParse {
            id: 20,
            entry_error: entry::Error::EmptyName
        }
    )
}

/// `core_data/empty_path.sql`: an empty `ZPATH` is a parse error that carries
/// the token's id.
#[test]
fn errors_on_empty_paths() {
    let (_dir, path) = CoreDataFixtureDb::EmptyPath.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(
        error,
        index::Error::RowParse {
            id: 20,
            entry_error: entry::Error::EmptyPath
        }
    )
}

/// `core_data/empty_table.sql`: all tables present but empty is an empty `Vec`,
/// not an error.
#[test]
fn an_empty_table_yields_an_empty_collection() {
    let (_dir, path) = CoreDataFixtureDb::EmptyTable.build();
    let entries = read_entries(&path).unwrap();
    assert!(entries.is_empty())
}

/// `core_data/missing_file.sql`: metadata pointing at a nonexistent `ZFILEPATH`
/// leaves `path` NULL, reported as a read error for that token, not dropped.
#[test]
fn errors_if_missing_filepath() {
    let (_dir, path) = CoreDataFixtureDb::MissingFile.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(
        error,
        index::Error::RowRead {
            id: 20,
            sqlite_error: _
        }
    )
}

/// `core_data/missing_metainformation.sql`: a token with no metadata row
/// leaves `path` NULL, reported as a read error for that token, not dropped.
#[test]
fn errors_if_missing_meta() {
    let (_dir, path) = CoreDataFixtureDb::MissingMeta.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(
        error,
        index::Error::RowRead {
            id: 20,
            sqlite_error: _
        }
    )
}

/// `core_data/missing_tables.sql`: only `ZTOKEN` exists, so the schema is
/// detected as Core Data but the query can't be prepared: a database error
/// with no row id.
#[test]
fn errors_if_missing_tables() {
    let (_dir, path) = CoreDataFixtureDb::MissingTables.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(error, index::Error::Database(_))
}

/// `core_data/missing_type.sql`: a token pointing at a nonexistent `ZTOKENTYPE`
/// leaves `type` NULL, reported as a read error for that token, not dropped.
#[test]
fn errors_if_missing_type() {
    let (_dir, path) = CoreDataFixtureDb::MissingType.build();
    let error = read_entries(&path).unwrap_err();
    std::assert_matches!(
        error,
        index::Error::RowRead {
            id: 20,
            sqlite_error: _
        }
    )
}

/// `core_data/null_anchor.sql`: two real CSS rows with a NULL `ZANCHOR` are
/// valid entries with `anchor: None`.
#[test]
fn uses_none_for_missing_anchor() {
    let (_dir, path) = CoreDataFixtureDb::NullAnchor.build();
    let entries = read_entries(&path).unwrap();
    let expected = [
        (
            "CSS writing modes",
            "Guide",
            "developer.mozilla.org/en-US/docs/Web/CSS/Guides/Writing_modes.html",
            None,
        ),
        (
            ":heading",
            "Class",
            "developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/_heading_function.html",
            None,
        ),
    ]
    .map(|(name, kind, path, anchor)| RawEntry {
        name,
        kind,
        path,
        anchor,
    })
    .map(|row| Entry::try_from(row).unwrap());
    assert_eq!(entries, expected);
}
