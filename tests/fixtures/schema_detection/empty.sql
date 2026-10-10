-- A valid SQLite database with no tables at all. Expected: 0 (unknown).
-- A script of only comments writes nothing, so SQLite may never create the
-- file. Setting `user_version` writes the database header without adding a
-- table, so a real, empty database file exists to open.
PRAGMA user_version = 1;
