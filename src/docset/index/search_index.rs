//! Reader for "schema A" indexes: a single SQLite table,
//! `searchIndex(id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT)`,
//! with no `NOT NULL` or `CHECK` constraints on any column.

use std::path::Path;

use rusqlite::{Connection, OpenFlags, Row};

use crate::docset::entry::{Entry, EntryError};

/// Why a schema-A index couldn't be read.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum IndexError {
    #[error("could not read the database")]
    Database(#[from] rusqlite::Error),
    #[error("could not read row (id: {id}) from the database")]
    RowRead {
        id: i64,
        #[source]
        sqlite_error: rusqlite::Error,
    },
    #[error("parse error reading row {id}")]
    RowParse {
        id: i64,
        #[source]
        entry_error: EntryError,
    },
}

/// Reads every row of the `searchIndex` table in the SQLite file at
/// `index_path` (a docset's `docSet.dsidx`), ordered by `id`.
///
/// Opens the file read-only. It must never create or modify it.
///
/// # Errors
///
/// Fails on the first problem; it never skips a bad row:
/// - the file is missing, or isn't a SQLite database;
/// - the database has no `searchIndex` table (e.g. a Core Data docset);
/// - a row can't be read (e.g. a `NULL` column) or converted into an `Entry`.
pub fn read_entries(index_path: &Path) -> Result<Vec<Entry>, IndexError> {
    let conn = Connection::open_with_flags(
        index_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut query = conn.prepare(include_str!("read_all.sql"))?;
    query.query(())?.and_then(read_row).collect()
}

/// Reads a single `searchIndex` database row into an [`Entry`].
///
/// Expects a valid row containing `id`, `name`, `type`, and `path` columns (see
/// `read_all.sql` for the actual query shape). Calls [`Entry::try_from`] with
/// the resultant values.
///
/// # Errors
///
/// Fails on the first read issue:
/// - [`IndexError::Database`] if the row's `id` cannot be read.
/// - [`IndexError::RowRead`] if the row's other columns cannot be read, e.g. a
///   `NULL`. `id` cannot use this as the `id` is not known if it errors.
/// - [`IndexError::RowParse`] if the values do not parse to [`Entry`].
fn read_row(row: &Row<'_>) -> Result<Entry, IndexError> {
    let id = row.get("id")?;

    let name: String = row
        .get("name")
        .map_err(|sqlite_error| IndexError::RowRead { id, sqlite_error })?;
    let entry_type: String = row
        .get("type")
        .map_err(|sqlite_error| IndexError::RowRead { id, sqlite_error })?;
    let path: String = row
        .get("path")
        .map_err(|sqlite_error| IndexError::RowRead { id, sqlite_error })?;

    Entry::try_from((name.as_str(), entry_type.as_str(), path.as_str()))
        .map_err(|entry_error| IndexError::RowParse { id, entry_error })
}
