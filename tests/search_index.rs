mod common;

use dashtui::docset::entry;
use dashtui::docset::entry::Entry;
use dashtui::docset::index::{self, read_entries};
use rusqlite::Error;

use crate::common::{FixtureDatabase, SearchIndexFixtureDb};

/// Every row of `valid.sql` comes back as an `Entry`, in `id` order, equal to
/// what `Entry::try_from` makes of the same row.
#[test]
fn reads_every_row_of_a_valid_index() {
    let (_dir, path) = SearchIndexFixtureDb::Valid.build();

    let entries = read_entries(&path).unwrap();

    let expected = [
        (
            ":",
            "Builtin",
            "bash/Bourne-Shell-Builtins.html",
            Some("//apple_ref/Builtin/%3A"),
        ),
        ("Introduction", "Guide", "bash/Introduction.html", None),
        (
            "!",
            "Word",
            "bash/Pipelines.html",
            Some("//apple_ref/Word/%21"),
        ),
        (
            "abort (C-g)",
            "Function",
            "bash/Miscellaneous-Commands.html",
            Some("//apple_ref/Function/abort%20%28C%2Dg%29"),
        ),
        (
            "_",
            "Variable",
            "bash/Bash-Variables.html",
            Some("//apple_ref/Variable/%5F"),
        ),
        (
            "-",
            "Parameter",
            "bash/Special-Parameters.html",
            Some("//apple_ref/Parameter/%2D"),
        ),
    ]
    .map(|row| Entry::try_from(row).unwrap());
    assert_eq!(entries, expected);
}

/// A valid but empty database returns Ok() with no rows.
#[test]
fn returns_empty_vec_for_empty_table() {
    let (_dir, path) = SearchIndexFixtureDb::EmptyTable.build();
    let entries = read_entries(&path).unwrap();

    assert!(entries.is_empty());
}

/// A path with no file behind it is an error, and the file must still not
/// exist afterwards: reading an index must never create one.
#[test]
fn missing_file_is_an_error_and_is_not_created() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docSet.dsidx");
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(err, index::Error::Database(_));
    assert!(!path.exists());
}

/// A file that exists but isn't a SQLite database is an error.
#[test]
fn non_database_file_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docSet.dsidx");
    std::fs::write(&path, "Hello world").unwrap();

    let err = read_entries(&path).unwrap_err();
    std::assert_matches!(err, index::Error::Database(_));
}

/// `no_search_index.sql`: a real database with no `searchIndex` table is an
/// error, not an empty `Vec`.
#[test]
fn database_without_search_index_table_is_an_error() {
    let (_dir, path) = SearchIndexFixtureDb::NoSearchIndex.build();
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(err, index::Error::Database(_));
}

/// `empty_path.sql`: the error is your row-level variant, carrying `id` 20
/// (not 2, the row's position) and `entry::Error::EmptyPath`.
#[test]
fn invalid_row_error_carries_its_row_id() {
    let (_dir, path) = SearchIndexFixtureDb::EmptyPath.build();
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(
        err,
        index::Error::RowParse {
            id: 20,
            entry_error: entry::Error::EmptyPath
        }
    );
}

/// `null_path.sql`: a `NULL` column is an error.
#[test]
fn null_column_is_an_error() {
    let (_dir, path) = SearchIndexFixtureDb::NullPath.build();
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(
        err,
        index::Error::RowRead {
            id: 20,
            sqlite_error: Error::InvalidColumnType(_, _, _)
        }
    );
}
