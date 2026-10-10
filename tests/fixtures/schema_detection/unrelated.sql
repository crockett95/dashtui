-- A valid SQLite database whose tables belong to neither schema.
-- Expected: 0 (unknown).
CREATE TABLE app_setting (setting_name TEXT PRIMARY KEY, setting_value TEXT);
CREATE TABLE search_log (id INTEGER PRIMARY KEY, query_text TEXT);
