-- Query all the entries from a `searchIndex` table
SELECT
    s.id,
    s.name,
    s.type,
    s.path
FROM searchIndex AS s
ORDER BY s.id;
