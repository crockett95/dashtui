-- Row id 20 has a NULL path, which the schema permits (no NOT NULL constraint)
-- but no real docset surveyed has ever contained.
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
INSERT INTO searchIndex VALUES (
    10, 'echo', 'Builtin', 'bash/Bash-Builtins.html'
);
INSERT INTO searchIndex VALUES (20, 'printf', 'Builtin', NULL);
