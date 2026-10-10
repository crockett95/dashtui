-- Only `ZTOKEN` exists, so detection says Core Data but the read query can't be
-- prepared (no such table: ZTOKENMETAINFORMATION). Expected: a database error,
-- with no row id.
CREATE TABLE ZTOKEN (
    Z_PK INTEGER PRIMARY KEY, ZTOKENTYPE INTEGER, ZTOKENNAME VARCHAR
);
INSERT INTO ZTOKEN VALUES (10, 1, 'print');
