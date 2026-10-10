-- SearchIndex: a `searchIndex` table and nothing else (Bash.docset,
-- PHP.docset).
-- Expected: 1 (search index).
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
