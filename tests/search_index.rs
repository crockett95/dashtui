use std::path::PathBuf;

use dashtui::docset::entry::{Entry, EntryError};
use dashtui::docset::index::search_index::{IndexError, read_entries};
use rusqlite::{Connection, Error};
use tempfile::TempDir;

/// Builds a real SQLite file from a fixture's SQL, inside a fresh temp dir.
///
/// Returns the `TempDir` along with the file's path because the directory
/// is deleted when the `TempDir` is dropped: callers must keep it alive for
/// the whole test with `let (_dir, path) = ...`. Writing `let (_, path)`
/// instead would drop it, and delete the file, immediately.
fn fixture_db(sql: &str) -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docSet.dsidx");
    let conn = Connection::open(&path).unwrap();
    conn.execute_batch(sql).unwrap();
    (dir, path)
}

/// Every row of `valid.sql` comes back as an `Entry`, in `id` order, equal to
/// what `Entry::try_from` makes of the same row.
#[test]
fn reads_every_row_of_a_valid_index() {
    let (_dir, path) = fixture_db(include_str!("fixtures/search_index/valid.sql"));

    let entries = read_entries(&path).unwrap();

    let expected = [
        (
            ":",
            "Builtin",
            "bash/Bourne-Shell-Builtins.html#//apple_ref/Builtin/%3A",
        ),
        ("Introduction", "Guide", "bash/Introduction.html"),
        ("!", "Word", "bash/Pipelines.html#//apple_ref/Word/%21"),
        (
            "abort (C-g)",
            "Function",
            "bash/Miscellaneous-Commands.html#//apple_ref/Function/abort%20%28C%2Dg%29",
        ),
        (
            "_",
            "Variable",
            "bash/Bash-Variables.html#//apple_ref/Variable/%5F",
        ),
        (
            "-",
            "Parameter",
            "bash/Special-Parameters.html#//apple_ref/Parameter/%2D",
        ),
    ]
    .map(|row| Entry::try_from(row).unwrap());
    assert_eq!(entries, expected);
}

/// A valid but empty database returns Ok() with no rows.
#[test]
fn returns_empty_vec_for_empty_table() {
    let (_dir, path) = fixture_db(include_str!("fixtures/search_index/empty_table.sql"));
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

    std::assert_matches!(err, IndexError::Database(_));
    assert!(!path.exists());
}

/// A file that exists but isn't a SQLite database is an error.
#[test]
fn non_database_file_is_an_error() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("docSet.dsidx");
    std::fs::write(&path, "Hello world").unwrap();

    let err = read_entries(&path).unwrap_err();
    std::assert_matches!(err, IndexError::Database(_));
}

/// `no_search_index.sql`: a real database with no `searchIndex` table is an
/// error, not an empty `Vec`.
#[test]
fn database_without_search_index_table_is_an_error() {
    let (_dir, path) = fixture_db(include_str!("fixtures/search_index/no_search_index.sql"));
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(err, IndexError::Database(_));
}

/// `empty_path.sql`: the error is your row-level variant, carrying `id` 20
/// (not 2, the row's position) and `EntryError::EmptyPath`.
#[test]
fn invalid_row_error_carries_its_row_id() {
    let (_dir, path) = fixture_db(include_str!("fixtures/search_index/empty_path.sql"));
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(
        err,
        IndexError::RowParse {
            id: 20,
            entry_error: EntryError::EmptyPath
        }
    );
}

/// `null_path.sql`: a `NULL` column is an error.
#[test]
fn null_column_is_an_error() {
    let (_dir, path) = fixture_db(include_str!("fixtures/search_index/null_path.sql"));
    let err = read_entries(&path).unwrap_err();

    std::assert_matches!(
        err,
        IndexError::RowRead {
            id: 20,
            sqlite_error: Error::InvalidColumnType(_, _, _)
        }
    );
}
