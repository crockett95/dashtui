-- Both schemas present as real tables. No real docset seen does this; it pins
-- the precedence rule. Expected: 2 (Core Data wins).
CREATE TABLE searchIndex (
    id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT
);
CREATE TABLE ZTOKEN (Z_PK INTEGER PRIMARY KEY, ZTOKENNAME VARCHAR);
