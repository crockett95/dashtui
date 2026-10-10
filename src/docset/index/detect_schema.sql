-- Detect the docset schema from what tables it has. Numbering is explicitly
-- set to give precedence to Core Data and to work with an implementation using
-- `MAX`, should the need ever arise to switch.
-- - 0: Unknown
-- - 1: SearchIndex (a `searchIndex` table)
-- - 2: Core Data (a `ZTOKEN` table)
SELECT CASE
    WHEN -- Core Data
        EXISTS (
            SELECT 1 FROM pragma_table_list('ZTOKEN')
            WHERE type = 'table'
        )
        THEN 2
    WHEN -- SearchIndex
        EXISTS (
            SELECT 1 FROM pragma_table_list('searchIndex')
            WHERE type = 'table'
        )
        THEN 1
    ELSE 0
END AS schema_type
