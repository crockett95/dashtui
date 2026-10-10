-- SearchIndex, but the table name is spelled in a different case. SQLite table
-- names are case-insensitive, so `SELECT ... FROM searchIndex` reads this
-- table fine, and detection resolves the name the same way
-- (`pragma_table_list`). Expected: 1 (search index).
CREATE TABLE SEARCHINDEX (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
