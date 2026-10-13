-- Query all the records in an index for search, stats, etc
--
SELECT
    T.Z_PK AS id,
    T.ZTOKENNAME AS name,
    TT.ZTYPENAME AS type,
    F.ZPATH AS path,
    M.ZANCHOR AS anchor
FROM ZTOKEN AS T
-- Left outer joins allow dealing with failed joins in code
LEFT OUTER JOIN ZTOKENMETAINFORMATION AS M ON T.Z_PK = M.ZTOKEN
LEFT OUTER JOIN ZTOKENTYPE AS TT ON T.ZTOKENTYPE = TT.Z_PK
LEFT OUTER JOIN ZFILEPATH AS F ON M.ZFILE = F.Z_PK
ORDER BY T.Z_PK
