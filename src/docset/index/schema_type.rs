//! The index schemas docsets use in the wild, and the SQL each one needs.
//!
//! Everything schema-specific lives here: which schema an index uses, and
//! which query reads it. Callers get prepared statements back, and everything
//! after that is shared (ADR-8).
use rusqlite::{Connection, Statement};

/// Which on-disk schema a docset's index uses.
///
/// See the [`index`](super) module docs for what each schema looks like.
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum SchemaType {
    /// Entries live in a single `searchIndex` table.
    SearchIndex,
    /// Entries are assembled from Apple Core Data tables (`ZTOKEN` and its
    /// related `Z`-prefixed tables).
    CoreData,
}

/// Why a schema code couldn't be turned into a [`SchemaType`].
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum Error {
    /// The code isn't one [`SchemaType::detect_schema`] produces for a known
    /// schema. Code `0` means the database matched neither schema.
    #[error("invalid schema type")]
    InvalidSchema,
}

/// Converts a code returned by [`SchemaType::detect_schema`]'s query.
///
/// `1` is [`SchemaType::SearchIndex`] and `2` is [`SchemaType::CoreData`].
/// Anything else, including `0` ("neither schema"), is
/// [`Error::InvalidSchema`].
impl TryFrom<i64> for SchemaType {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::SearchIndex),
            2 => Ok(Self::CoreData),
            _ => Err(Self::Error::InvalidSchema),
        }
    }
}

impl SchemaType {
    /// Prepares the query that detects which schema the database on `conn`
    /// uses.
    ///
    /// The query returns exactly one row with one column, `schema_type`: `0`
    /// for neither schema, `1` for `SearchIndex`, `2` for Core Data. Convert it
    /// with [`SchemaType::try_from`]. Detection only counts real tables, not
    /// views, and resolves names the way SQLite does (case-insensitively).
    /// If both schemas' tables exist, Core Data wins.
    ///
    /// # Errors
    ///
    /// Returns the SQLite error if the query can't be prepared, for example
    /// because the file isn't a SQLite database.
    pub fn detect_schema(conn: &'_ Connection) -> rusqlite::Result<Statement<'_>> {
        conn.prepare(include_str!("detect_schema.sql"))
    }

    /// Prepares this schema's query for reading every entry from the database
    /// on `conn`.
    ///
    /// Every schema's query returns the columns the shared row reader expects
    /// (`id`, `name`, `type`, `path`, `anchor`), so rows from any schema are
    /// read the same way. `path` is the page and `anchor` the part after its
    /// `#`, already separated; `anchor` is `NULL` for an entry without one.
    ///
    /// # Errors
    ///
    /// Returns the SQLite error if the query can't be prepared, for example
    /// because the tables this schema needs don't exist.
    pub fn read_all(self, conn: &Connection) -> rusqlite::Result<Statement<'_>> {
        match self {
            SchemaType::SearchIndex => conn.prepare(include_str!("read_search_index_all.sql")),
            SchemaType::CoreData => conn.prepare(include_str!("read_core_all.sql")),
        }
    }
}

#[cfg(test)]
mod test {

    use std::collections::HashSet;

    use rusqlite::fallible_iterator::FallibleIterator;

    use super::*;

    #[test]
    fn maps_1_to_search_index() {
        let val = SchemaType::try_from(1).unwrap();
        assert_eq!(SchemaType::SearchIndex, val);
    }

    #[test]
    fn maps_2_to_core_data() {
        let val = SchemaType::try_from(2).unwrap();
        assert_eq!(SchemaType::CoreData, val);
    }

    #[test]
    fn maps_higher_values_to_error() {
        for val in 3..100 {
            let err = SchemaType::try_from(val).unwrap_err();
            std::assert_matches!(err, Error::InvalidSchema);
        }
    }

    #[test]
    fn maps_lower_values_to_error() {
        for val in -100..1 {
            let err = SchemaType::try_from(val).unwrap_err();
            std::assert_matches!(err, Error::InvalidSchema);
        }
    }

    macro_rules! fixture_db {
        ($name:literal) => {
            include_str!(concat!("../../../tests/fixtures/", $name))
        };
    }

    fn fixture_conn(sql: &str) -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(sql).unwrap();
        conn
    }

    fn detect_fixture_schema(sql: &str) -> Result<SchemaType, Error> {
        let conn = fixture_conn(sql);

        let mut query = SchemaType::detect_schema(&conn).unwrap();
        let raw: i64 = query.query_one((), |r| r.get("schema_type")).unwrap();
        SchemaType::try_from(raw)
    }

    #[test]
    fn detects_search_index_schema() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/search_index.sql")).unwrap();
        assert_eq!(result, SchemaType::SearchIndex);
    }

    #[test]
    fn detects_search_index_schema_if_uppercase() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/search_index_uppercase.sql"))
                .unwrap();
        assert_eq!(result, SchemaType::SearchIndex);
    }

    #[test]
    fn detects_core_data_schema() {
        let result = detect_fixture_schema(fixture_db!("schema_detection/core_data.sql")).unwrap();
        assert_eq!(result, SchemaType::CoreData);
    }

    #[test]
    fn detects_core_data_schema_with_zeal_view() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/core_data_with_zeal_view.sql"))
                .unwrap();
        assert_eq!(result, SchemaType::CoreData);
    }

    #[test]
    fn detects_core_data_schema_if_both_tables() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/both_tables.sql")).unwrap();
        assert_eq!(result, SchemaType::CoreData);
    }

    #[test]
    fn detects_invalid_schema_for_empty_db() {
        let result = detect_fixture_schema(fixture_db!("schema_detection/empty.sql")).unwrap_err();
        assert_eq!(result, Error::InvalidSchema);
    }

    #[test]
    fn detects_invalid_schema_for_view_only() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/search_index_view_only.sql"))
                .unwrap_err();
        assert_eq!(result, Error::InvalidSchema);
    }

    #[test]
    fn detects_invalid_schema_for_unrelated_schema() {
        let result =
            detect_fixture_schema(fixture_db!("schema_detection/unrelated.sql")).unwrap_err();
        assert_eq!(result, Error::InvalidSchema);
    }

    /// Asserts that an entry-reading query follows ADR-8's shared-reader
    /// contract: it returns exactly the columns the shared row reader reads
    /// (`id`, `name`, `type`, `path`, `anchor`), no more and no fewer.
    macro_rules! assert_has_entry_columns {
        ($query:expr) => {
            let columns: HashSet<_> = $query.column_names().into_iter().collect();
            let expected = HashSet::from(["id", "name", "type", "path", "anchor"]);

            assert_eq!(columns, expected)
        };
    }

    #[test]
    fn reads_expected_cols_of_search_index_schema() {
        let conn = fixture_conn(fixture_db!("search_index/valid.sql"));

        let query = SchemaType::SearchIndex.read_all(&conn).unwrap();
        assert_has_entry_columns!(query);
    }

    #[test]
    fn fails_to_read_expected_cols_of_invalid_search_index() {
        let conn = fixture_conn(fixture_db!("search_index/no_search_index.sql"));

        let err = SchemaType::SearchIndex.read_all(&conn).unwrap_err();
        std::assert_matches!(err, rusqlite::Error::SqliteFailure(_, _));
    }

    #[test]
    fn reads_expected_cols_of_valid_core_data_schema() {
        let conn = fixture_conn(fixture_db!("core_data/valid.sql"));

        let query = SchemaType::CoreData.read_all(&conn).unwrap();
        assert_has_entry_columns!(query);
    }

    #[test]
    fn fails_to_read_expected_cols_of_invalid_core_data_schema() {
        let conn = fixture_conn(fixture_db!("core_data/missing_tables.sql"));

        let err = SchemaType::CoreData.read_all(&conn).unwrap_err();
        std::assert_matches!(err, rusqlite::Error::SqliteFailure(_, _));
    }

    #[test]
    fn splits_search_index_anchors() {
        let conn = fixture_conn(fixture_db!("search_index/anchors.sql"));

        let mut statement = SchemaType::SearchIndex.read_all(&conn).unwrap();
        let rows: Vec<(String, String)> = statement
            .query(())
            .unwrap()
            .map(|r| Ok((r.get_unwrap("path"), r.get_unwrap("anchor"))))
            .unwrap()
            .collect();
        let expected = [
            ("sqlite/printf.html".to_string(), "percentq".to_string()),
            ("sqlite/fileformat2.html".to_string(), "#sqltab".to_string()),
        ];
        assert_eq!(rows, expected);
    }
}
