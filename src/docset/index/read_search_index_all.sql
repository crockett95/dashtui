-- Query all the entries from a `searchIndex` table
SELECT
    s.id,
    s.name,
    s.type,
    s.path,
    NULL AS fragment
FROM searchIndex AS s
ORDER BY s.id;
