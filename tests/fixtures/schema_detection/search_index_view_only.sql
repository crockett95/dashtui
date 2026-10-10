-- Something named `searchIndex` exists, but only as a view, with no real
-- table behind either schema. Expected: 0 (unknown).
CREATE TABLE app_setting (setting_name TEXT PRIMARY KEY, setting_value TEXT);
CREATE VIEW searchIndex AS
SELECT
    1 AS id,
    setting_name AS name,
    'Guide' AS type,
    setting_value AS path
FROM app_setting;
