# Docset entry `type` conventions

There's no formal spec for a docset entry's `type` (schema A: `searchIndex.type`;
Core Data / schema B: `ZTOKENTYPE.ZTYPENAME`) — it's free text chosen by whatever
generator built the docset (`dashing`, `doc2dash`, hand-rolled scripts...), with no
`CHECK` constraint, foreign key, or lookup table backing it in either schema
(verified via `sqlite3 .schema` against `docsets/`; see `PROGRESS.md`, M3). This page
collects what was learned empirically while designing `EntryType`
(`src/docset/entry.rs`, decision 9 in `PLAN.md`), so it doesn't need re-deriving.

**Compiled 2026-09-30.**

## What `EntryType` actually implements right now

Scoped deliberately to only what the 3 real docsets in `docsets/` need — see decision
9 in `PLAN.md` for why. Everything else below is reference material for extending it.

| Variant | Raw strings it matches |
| --- | --- |
| `Builtin` | `Builtin` |
| `Function` | `Function`, `func` |
| `Guide` | `Guide` |
| `Macro` | `Macro`, `macro` |
| `Parameter` | `Parameter` |
| `Variable` | `Variable` |
| `Word` | `Word` |
| `Other(String)` | anything else, verbatim |

## Zeal's full canonical vocabulary + alias table

Source: `zealdocs/zeal`, `src/libs/registry/docset.cpp`,
`Docset::parseSymbolType` — read directly from a shallow clone, not from memory.
Zeal doesn't use a closed enum either: `parseSymbolType` returns a plain `QString`,
normalized through a hand-maintained `QHash<QString, QString>` alias table
(`aliases.value(str, str)` — returns the input **unchanged** if not found, so
anything unrecognized just passes through as itself). The table below is the
complete set of ~20 canonical target names and every alias mapped onto each, as of
the commit cloned.

| Canonical name | Raw strings that normalize to it |
| --- | --- |
| `Attribute` | `Package Attributes`, `Private Attributes`, `Protected Attributes`, `Public Attributes`, `Static Package Attributes`, `Static Private Attributes`, `Static Protected Attributes`, `Static Public Attributes`, `XML Attributes` |
| `Binding` | `binding` |
| `Category` | `cat`, `Groups`, `Pages` |
| `Class` | `cl`, `specialization`, `tmplt` |
| `Constant` | `data`, `econst`, `enumdata`, `enumelt`, `clconst`, `structdata`, `writerid`, `Notifications` |
| `Constructor` | `structctr`, `Public Constructors` |
| `Enumeration` | `enum`, `Enum`, `Enumerations` |
| `Event` | `event`, `Public Events`, `Inherited Events`, `Private Events` |
| `Field` | `Data Fields` |
| `Function` | `dcop`, `func`, `ffunc`, `signal`, `slot`, `grammar`, `Function Prototypes`, `Functions/Subroutines`, `Members`, `Package Functions`, `Private Member Functions`, `Private Slots`, `Protected Member Functions`, `Protected Slots`, `Public Member Functions`, `Public Slots`, `Signals`, `Static Package Functions`, `Static Private Member Functions`, `Static Protected Member Functions`, `Static Public Member Functions` |
| `Guide` | `doc` |
| `Namespace` | `ns` |
| `Macro` | `macro` |
| `Method` | `clm`, `enumcm`, `enumctr`, `enumm`, `intfctr`, `intfcm`, `intfm`, `intfsub`, `instsub`, `instctr`, `instm`, `structcm`, `structm`, `structsub`, `Class Methods`, `Inherited Methods`, `Instance Methods`, `Private Methods`, `Protected Methods`, `Public Methods` |
| `Operator` | `intfopfunc`, `opfunc` |
| `Property` | `enump`, `intfdata`, `intfp`, `instp`, `structp`, `Inherited Properties`, `Private Properties`, `Protected Properties`, `Public Properties` |
| `Protocol` | `intf` |
| `Structure` | `_Struct`, `_Structs`, `struct`, `Control Structure`, `Data Structures`, `Struct` |
| `Type` | `tag`, `tdef`, `Data Types`, `Package Types`, `Private Types`, `Protected Types`, `Public Types`, `Typedefs` |
| `Variable` | `var` |

Note the `cl`/`clm`/`clconst`/`instp`/`tdef` family — Xcode/Doxygen-generator
shorthand (class / class-method / class-constant / instance-property / typedef)
that shows up unexplained in real docsets (see survey below) until you know Zeal
already decoded it.

## 23-docset survey: real-world `type` usage

Pulled from `github.com/Kapeli/feeds` (the same source M0 used to get
`Bash`/`SQLite`/`Lua`): Python, JavaScript, TypeScript, Java, C, C++, Go, Rust, Ruby,
PHP, Swift, React, Django, Ruby_on_Rails, VueJS, NodeJS, PostgreSQL, MySQL, Redis,
Docker, plus the existing Bash/SQLite/Lua. All 23 exposed a `searchIndex` table (9 of
the 20 new ones — C++, C, Django, Java, JavaScript, MySQL, Python, Ruby_on_Rails —
*also* carried Core Data tables in the same file; schema isn't strictly either/or).

**183 (docset, type) rows, 55 distinct type strings. 36 shared verbatim by 2+
docsets; 19 unique to exactly one docset.**

Full table (count per docset that uses that string), sorted by how many docsets share it:

```
18  Guide       C++:388 C:161 Django:276 Docker:1261 Go:127 JavaScript:690 NodeJS:908 PHP:144 PostgreSQL:35 Python:250 React:178 Redis:1583 Ruby:66 Ruby_on_Rails:66 Rust:1170 TypeScript:71 VueJS:81 Bash:139
12  Section     Django:1637 Docker:8472 Java:670 MySQL:2435 Python:1325 React:1186 Redis:6395 Ruby:7 Rust:5710 Swift:5 TypeScript:362 VueJS:402
9   Function    Django:232 Go:9992 MySQL:1715 PHP:73 PostgreSQL:2421 Python:4060 Rust:606 Swift:31 Bash:116
8   Variable    Django:7 Go:676 MySQL:2208 NodeJS:1 PHP:13 Python:89 Swift:1171 Bash:163
8   Attribute   C++:11 C:7 Django:1044 Python:1436 Ruby:851 Ruby_on_Rails:601 Rust:7 VueJS:3
7   Module      Django:143 Java:95 NodeJS:96 Python:316 Ruby:166 Ruby_on_Rails:641 Rust:159
7   Method      Django:948 Java:39398 Python:5621 Ruby:11664 Rust:50186 Swift:7481 VueJS:155
7   Constant    Go:6562 Java:1553 PHP:2060 Python:148 Ruby:1876 Rust:481 Swift:66
6   func        C++:1425 C:408 JavaScript:12 PHP:3962 SQLite:2296 Lua:461
5   clm         C++:2341 JavaScript:3068 NodeJS:2077 PHP:5599 Ruby_on_Rails:4847
5   cl          C++:1155 JavaScript:1125 NodeJS:296 PHP:739 Ruby_on_Rails:728
4   Type        PostgreSQL:198 Python:128 Rust:11920 Swift:169
4   Option      Django:148 MySQL:88 NodeJS:217 Python:334
4   Keyword     C++:100 C:59 JavaScript:107 Rust:40
4   Enum        C++:30 C:2 Python:5 Rust:81
4   Class       Django:744 Java:3322 Python:1036 Ruby:999
3   tdef        C++:264 C:89 Go:2104
3   Struct      C++:9 C:7 Python:9
3   Setting     Django:216 PHP:390 PostgreSQL:433
3   Property    Java:201 JavaScript:3678 PHP:763
3   Operator    C++:688 MySQL:86 Swift:56
3   Macro       MySQL:21 Python:427 Rust:102
3   macro       C++:452 C:394 SQLite:715
3   instp       C++:2 C:1 NodeJS:829
3   Field       Django:104 Java:7245 Rust:197
3   Error       MySQL:5932 NodeJS:427 PostgreSQL:262
3   Command     Django:34 Docker:522 Redis:600
3   clconst     C++:215 C:1 Ruby_on_Rails:479
2   Tag         C++:13 Django:47
2   Package     Go:394 Java:757
2   Interface   Java:1656 PHP:58
2   File        C++:126 C:32
2   Exception   Django:53 Python:312
2   Event       JavaScript:461 NodeJS:249
2   Directive   C++:2 VueJS:16
2   Constructor Java:5018 JavaScript:312
1   Word        Bash:21
1   View        PostgreSQL:64
1   Variant     Rust:447
1   Union       Rust:1
1   Trait       Rust:229
1   _Struct     Rust:571
1   Statement   Python:21
1   Sample      Go:932
1   Query       PostgreSQL:476
1   Protocol    Swift:63
1   Procedure   PostgreSQL:56
1   instm       Ruby_on_Rails:930
1   Global      C++:4
1   Filter      Django:72
1   Element     Java:41
1   Component   VueJS:5
1   Builtin     Bash:61
1   Alias       Swift:425
```

### Notable findings (not just spelling drift)

- **`PHP.docset` uses both `func` (3962 rows) and `Function` (73 rows)
  simultaneously** — not a cross-docset spelling difference, the same docset uses
  both. No way to tell from outside whether that's intentional (two real concepts)
  or generator inconsistency. Zeal's alias table merges `func → Function`
  unconditionally regardless; we don't currently alias `Function` vs `func` beyond
  what our own docsets need (both map to `EntryType::Function`).
- **`C++.docset` mixes three incompatible naming conventions in one file**:
  spelled-out capitalized (`Guide`, `Enum`, `Struct`, `Keyword`...), lowercase-short
  (`func`, `macro`), and the cryptic abbreviated family (`cl`, `clm`, `clconst`,
  `instp`, `tdef`) — all at once.
- **`Lua.docset` types every single one of its 461 `ZTOKEN` rows as `"func"`** —
  re-verified directly (row counts match, zero rows have a dangling/missing
  `ZTOKENTYPE` reference), not a query artifact. The docset genuinely doesn't
  differentiate entry types at all.

## Caveats

- Not exhaustive — 23 docsets and one shipped tool's alias table, not the whole
  ecosystem. Treat an unfamiliar string as "not yet seen," not "doesn't exist."
- `cl`/`clm`/`clconst`/`instp`/`tdef` reads like Xcode/Doxygen-generator shorthand,
  but that's inferred from Zeal's naming, not independently confirmed against a
  generator's own source.
- The near-miss clusters above (`func`/`Function`, `macro`/`Macro`) are flagged, not
  resolved — whether/how far to extend `EntryType`'s alias list beyond what
  `docsets/` needs is an open follow-up (see decision 9, `PLAN.md`).

## Sources

- `zealdocs/zeal`, `src/libs/registry/docset.cpp` (`Docset::parseSymbolType`) —
  cloned shallow and read directly, not from memory.
- `github.com/Kapeli/feeds` — same docset feed source `PROGRESS.md` (M0) used for
  `Bash`/`SQLite`/`Lua`.
- Real docsets in `docsets/` (gitignored) and the 20 surveyed above (not kept
  locally — downloaded to a scratch directory and discarded after the survey).
