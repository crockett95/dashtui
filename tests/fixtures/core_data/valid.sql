-- Three real rows from Lua.docset, verbatim (only the columns the reader uses).
-- Token ids 2, 140, 293 are deliberately not 1, 2, 3, and the metainformation
-- ids (402, 355, 1) differ from the token ids, so a reader that reports a
-- row's position, or joins on the wrong id, gets caught.
-- Expected: 3 entries, all type `func`, path
-- `www.lua.org/manual/5.5/manual.html`, each with its own anchor.
CREATE TABLE ZTOKEN (
    Z_PK INTEGER PRIMARY KEY, ZTOKENTYPE INTEGER, ZTOKENNAME VARCHAR
);
CREATE TABLE ZTOKENTYPE (Z_PK INTEGER PRIMARY KEY, ZTYPENAME VARCHAR);
CREATE TABLE ZTOKENMETAINFORMATION (
    Z_PK INTEGER PRIMARY KEY, ZFILE INTEGER, ZTOKEN INTEGER, ZANCHOR VARCHAR
);
CREATE TABLE ZFILEPATH (Z_PK INTEGER PRIMARY KEY, ZPATH VARCHAR);
INSERT INTO ZTOKENTYPE VALUES (1, 'func');
INSERT INTO ZFILEPATH VALUES (1, 'www.lua.org/manual/5.5/manual.html');
INSERT INTO ZTOKEN VALUES (2, 1, 'lua_pushvfstring');
INSERT INTO ZTOKEN VALUES (140, 1, 'LUA_LOADLIBK');
INSERT INTO ZTOKEN VALUES (293, 1, 'utf8.codes');
INSERT INTO ZTOKENMETAINFORMATION VALUES (
    402, 1, 2, '//apple_ref/func/lua%5Fpushvfstring'
);
INSERT INTO ZTOKENMETAINFORMATION VALUES (
    355, 1, 140, '//apple_ref/func/LUA%5FLOADLIBK'
);
INSERT INTO ZTOKENMETAINFORMATION VALUES (
    1, 1, 293, '//apple_ref/func/utf8%2Ecodes'
);
