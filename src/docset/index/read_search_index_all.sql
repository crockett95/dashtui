-- Query all the entries from a `searchIndex` table
SELECT
    s.id,
    s.name,
    s.type,
    CASE
        WHEN
            instr(s.path, '#')
            THEN substring(s.path, 1, instr(s.path, '#') - 1)
        ELSE s.path
    END AS path,
    CASE
        WHEN instr(s.path, '#') THEN substring(s.path, instr(s.path, '#') + 1)
    END AS fragment
FROM searchIndex AS s
ORDER BY s.id;
