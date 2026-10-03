-- Six real rows from Bash.docset (the first row of each type), verbatim.
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
INSERT INTO searchIndex VALUES (
    1, ':', 'Builtin', 'bash/Bourne-Shell-Builtins.html#//apple_ref/Builtin/%3A'
);
INSERT INTO searchIndex VALUES (
    62, 'Introduction', 'Guide', 'bash/Introduction.html'
);
INSERT INTO searchIndex VALUES (
    201, '!', 'Word', 'bash/Pipelines.html#//apple_ref/Word/%21'
);
INSERT INTO searchIndex VALUES (
    222,
    'abort (C-g)',
    'Function',
    'bash/Miscellaneous-Commands.html#//apple_ref/Function/abort%20%28C%2Dg%29'
);
INSERT INTO searchIndex VALUES (
    338, '_', 'Variable', 'bash/Bash-Variables.html#//apple_ref/Variable/%5F'
);
INSERT INTO searchIndex VALUES (
    339,
    '-',
    'Parameter',
    'bash/Special-Parameters.html#//apple_ref/Parameter/%2D'
);
