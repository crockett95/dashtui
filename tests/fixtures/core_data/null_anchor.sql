-- Two real rows from CSS.docset, verbatim. Both have a NULL `ZANCHOR`, which is
-- normal for Core Data (all of CSS's and C++'s rows, most of JavaScript's).
-- Expected: 2 valid entries with no anchor, not an error.
CREATE TABLE ZTOKEN (
    Z_PK INTEGER PRIMARY KEY, ZTOKENTYPE INTEGER, ZTOKENNAME VARCHAR
);
CREATE TABLE ZTOKENTYPE (Z_PK INTEGER PRIMARY KEY, ZTYPENAME VARCHAR);
CREATE TABLE ZTOKENMETAINFORMATION (
    Z_PK INTEGER PRIMARY KEY, ZFILE INTEGER, ZTOKEN INTEGER, ZANCHOR VARCHAR
);
CREATE TABLE ZFILEPATH (Z_PK INTEGER PRIMARY KEY, ZPATH VARCHAR);
INSERT INTO ZTOKENTYPE VALUES (4, 'Class');
INSERT INTO ZTOKENTYPE VALUES (6, 'Guide');
INSERT INTO ZFILEPATH VALUES (
    170, 'developer.mozilla.org/en-US/docs/Web/CSS/Guides/Writing_modes.html'
);
INSERT INTO ZFILEPATH VALUES (
    337,
    -- A real path, longer than the line limit; kept verbatim.
    'developer.mozilla.org/en-US/docs/Web/CSS/Reference/Selectors/_heading_function.html'  -- noqa: LT05
);
INSERT INTO ZTOKEN VALUES (773, 6, 'CSS writing modes');
INSERT INTO ZTOKEN VALUES (976, 4, ':heading');
INSERT INTO ZTOKENMETAINFORMATION VALUES (10, 170, 773, NULL);
INSERT INTO ZTOKENMETAINFORMATION VALUES (3, 337, 976, NULL);
