-- Row id 20 has an empty path. The ids are deliberately not 1, 2, 3, so a
-- reader that reports a row's position instead of its real `id` gets caught.
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
INSERT INTO searchIndex VALUES (
    10, 'echo', 'Builtin', 'bash/Bash-Builtins.html'
);
INSERT INTO searchIndex VALUES (20, 'printf', 'Builtin', '');
INSERT INTO searchIndex VALUES (
    30, 'read', 'Builtin', 'bash/Bash-Builtins.html'
);
