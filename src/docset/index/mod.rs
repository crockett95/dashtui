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
    entry::{self, Entry, RawEntry},
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
        entry_error: entry::Error,
    },
}

/// Reads every entry from the docset index at `index_path` (a docset's
/// `docSet.dsidx`), whichever schema it uses, ordered by the index's own ids.
///
/// Opens the file read-only (it must never create or modify it), detects the
/// schema on that one connection, then reads with that schema's query and the
/// shared row reader (ADR-8).
///
/// # Errors
///
/// Fails on the first problem; it never skips a bad row:
/// - [`Error::Database`]: the file is missing or isn't a SQLite database, or
///   the detected schema's tables are incomplete;
/// - [`Error::Schema`]: the database matches neither schema;
/// - [`Error::RowRead`] / [`Error::RowParse`]: a row can't be read (e.g. a
///   `NULL` column, or a broken Core Data link) or converted into an `Entry`.
pub fn read_entries(index_path: &Path) -> Result<Vec<Entry>, Error> {
    let conn = Connection::open_with_flags(
        index_path,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )?;

    let mut schema_query = SchemaType::detect_schema(&conn)?;
    let detected: i64 = schema_query.query_one((), |row| row.get("schema_type"))?;
    let schema = SchemaType::try_from(detected)?;

    let mut query = schema.read_all(&conn)?;
    query.query(())?.and_then(read_entry_row).collect()
}

/// Reads a single docset database row into an [`Entry`].
///
/// Expects a row with `id`, `name`, `type`, `path`, and `anchor` columns, the
/// shape every schema's read-all query returns (ADR-8). `anchor` may be
/// `NULL`; the other columns may not. Calls [`Entry::try_from`] with the
/// resultant values.
///
/// # Errors
///
/// Fails on the first read issue:
/// - [`Error::Database`] if the row's `id` cannot be read.
/// - [`Error::RowRead`] if the row's other columns cannot be read,
///   e.g. a `NULL`. `id` cannot use this as the `id` is not known if it errors.
/// - [`Error::RowParse`] if the values do not parse to [`Entry`].
fn read_entry_row(row: &Row<'_>) -> Result<Entry, Error> {
    let id = row.get("id")?;
    let row_err = |sqlite_error| Error::RowRead { id, sqlite_error };
    let sql_err = |sqlite_error| row_err(rusqlite::Error::from(sqlite_error));

    let name = row
        .get_ref("name")
        .map(|v| v.as_str())
        .map_err(row_err)?
        .map_err(sql_err)?;
    let kind = row
        .get_ref("type")
        .map(|v| v.as_str())
        .map_err(row_err)?
        .map_err(sql_err)?;
    let path = row
        .get_ref("path")
        .map(|v| v.as_str())
        .map_err(row_err)?
        .map_err(sql_err)?;
    let anchor = row
        .get_ref("anchor")
        .map(|v| v.as_str_or_null())
        .map_err(row_err)?
        .map_err(sql_err)?;

    Entry::try_from(RawEntry {
        name,
        kind,
        path,
        anchor,
    })
    .map_err(|entry_error| Error::RowParse { id, entry_error })
}
