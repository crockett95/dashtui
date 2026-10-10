-- A Core Data docset after Zeal has opened it: Zeal's `createView()` adds a
-- `searchIndex` *view* over the Z tables. Expected: 2 (Core Data), not 1.
CREATE TABLE ZTOKEN (
    Z_PK INTEGER PRIMARY KEY, ZTOKENNAME VARCHAR, ZTOKENTYPE INTEGER
);
CREATE TABLE ZTOKENTYPE (Z_PK INTEGER PRIMARY KEY, ZTYPENAME VARCHAR);
CREATE VIEW searchIndex AS
SELECT
    t.Z_PK AS id,
    t.ZTOKENNAME AS name,
    tt.ZTYPENAME AS type,
    NULL AS path
FROM ZTOKEN AS t
LEFT OUTER JOIN ZTOKENTYPE AS tt ON t.ZTOKENTYPE = tt.Z_PK;
