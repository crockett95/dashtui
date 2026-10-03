-- A valid SQLite database with no `searchIndex` table, shaped like a Core Data
-- docset (Lua.docset has ZTOKEN, ZTOKENTYPE, ZFILEPATH, ... and nothing else).
CREATE TABLE ZTOKEN (Z_PK INTEGER PRIMARY KEY, ZTOKENNAME VARCHAR);
INSERT INTO ZTOKEN VALUES (1, 'print');
