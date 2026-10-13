-- Real rows from SQLite.docset, verbatim, covering `#` anchors.
-- Expected (path, fragment) after reading, in id order:
--   8    one `#` ("sqlite/printf.html", "percentq")
--   2695 `##` splits at the first `#` ("sqlite/fileformat2.html", "#sqltab")
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
INSERT INTO searchIndex VALUES (
    2695, 'table data format', 'func', 'sqlite/fileformat2.html##sqltab'
);
INSERT INTO searchIndex VALUES (
    8, '%q', 'func', 'sqlite/printf.html#percentq'
);
