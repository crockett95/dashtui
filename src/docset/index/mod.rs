//! Reading a docset's index (`Contents/Resources/docSet.dsidx`).
//!
//! # Terminology
//!
//! - **Index**: a docset's SQLite database of entries, whatever its layout.
//!   This module means this.
//! - **SearchIndex schema**: an index whose entries live in a single
//!   `searchIndex` table. "SearchIndex" names the schema;
//!   `searchIndex`, in code formatting, names the table.
//! - **Core Data schema**: an index stored as Apple Core Data tables (`ZTOKEN`,
//!   `ZTOKENTYPE`, `ZFILEPATH`, ...), whose entries are assembled through joins.
//!
//! [`SchemaType`] supplies the prepared read-all statement; everything
//! after that (reading rows into entries) is one shared path (ADR-8).

pub mod schema_type;
use std::path::Path;

use rusqlite::{Connection, OpenFlags, Row};

use crate::docset::{
    entry::{Entry, EntryError},
    index::schema_type::SchemaType,
};

/// Why a docset-schema index couldn't be read.
#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("could not read the database")]
    Database(#[from] rusqlite::Error),
    #[error("invalid schema")]
    Schema(#[from] schema_type::Error),
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
pub fn read_entries(index_path: &Path) -> Result<Vec<Entry>, Error> {
    let conn = Connection::open_with_flags(
        index_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut query = SchemaType::SearchIndex.read_all(&conn)?;
    query.query(())?.and_then(read_row).collect()
}

/// Reads a single `searchIndex` database row into an [`Entry`].
///
/// Expects a valid row containing `id`, `name`, `type`, and `path` columns (see
/// `read_search_index_all.sql` for the actual query shape). Calls
/// [`Entry::try_from`] with the resultant values.
///
/// # Errors
///
/// Fails on the first read issue:
/// - [`Error::Database`] if the row's `id` cannot be read.
/// - [`Error::RowRead`] if the row's other columns cannot be read,
///   e.g. a `NULL`. `id` cannot use this as the `id` is not known if it errors.
/// - [`Error::RowParse`] if the values do not parse to [`Entry`].
fn read_row(row: &Row<'_>) -> Result<Entry, Error> {
    let id = row.get("id")?;

    let name: String = row
        .get("name")
        .map_err(|sqlite_error| Error::RowRead { id, sqlite_error })?;
    let entry_type: String = row
        .get("type")
        .map_err(|sqlite_error| Error::RowRead { id, sqlite_error })?;
    let path: String = row
        .get("path")
        .map_err(|sqlite_error| Error::RowRead { id, sqlite_error })?;

    Entry::try_from((name.as_str(), entry_type.as_str(), path.as_str()))
        .map_err(|entry_error| Error::RowParse { id, entry_error })
}
