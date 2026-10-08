# Architecture

How dashtui is put together. The reasons behind the structural choices are in
[adr.md](adr.md), cited below as `ADR-n`.

## Data flow

```
docsets dir --> discovery --> Docset (metadata + index reader + docs root)
                                   |
query --> search (fuzzy) --> Vec<Hit>
                                   |
Hit.path --> load HTML --> parse (HTML -> Vec<Block>) --> layout (wrap to width -> ratatui Lines)
                                   |
App state (mode, query, results, selection, doc, scroll, history)
   ^                                                  |
   +-- Msg events (keys, resize, worker results) <----+--> ui::draw(&App)
```

- **Discovery** finds every `*.docset` directory and reads its `Info.plist`
  ([docset-format.md](docset-format.md)). Docsets whose metadata won't parse
  are kept (ADR-6).
- **Index reading** turns a docset's SQLite index into `Entry` values. There
  are two on-disk schemas behind one trait (ADR-3), and entry types are
  normalized into `EntryType` (ADR-7, [entry-types.md](entry-types.md)).
  See [Terminology](#terminology) for what "index" and "SearchIndex" mean.
- **Rendering** happens in two stages: HTML is parsed once into `Block`s, and
  layout wraps them to the current width, so a resize doesn't parse again
  (ADR-1).
- **The TUI** follows the Elm pattern. `App::update` handles every `Msg`, and
  `ui::draw` only reads state (ADR-2). Slow work runs on a worker thread and
  sends its results back as `Msg`s (ADR-4).

## Layout

The project is one package with a library and a binary. Everything except
terminal I/O lives in the library, so it can be tested without a terminal.
`main.rs` stays thin: it parses arguments, chooses text or JSON output
(ADR-5), and runs the terminal.

```
src/
  lib.rs
  main.rs             clap CLI, terminal setup/teardown, event loop
  json.rs             --json view structs (ADR-5)
  stats.rs            entry counts by type
  docset/
    mod.rs            Docset, discovery of *.docset dirs
    meta.rs           Info.plist -> DocsetMeta
    entry.rs          Entry, EntryType
    index/
      mod.rs          schema detection (SchemaType, detect_schema)
      search_index.rs SearchIndex schema reader (one searchIndex table)
      *.sql           queries, loaded with include_str!
```

Planned, not built yet:

```
src/
  docset/index/core_data.rs  Core Data schema reader; IndexReader trait in index/mod.rs
  search.rs                  query -> ranked Hits
  render/
    parse.rs                 HTML -> Vec<Block>
    layout.rs                Blocks + width -> styled Lines, link spans
  app.rs                     App state + update(Msg), no terminal I/O
  ui.rs                      draw(&App, &mut Frame)
  worker.rs                  background thread + channels
  config.rs                  settings
```

## Terminology

- **Index**: a docset's SQLite database of entries
  (`Contents/Resources/docSet.dsidx`), whatever its layout. The
  `docset::index` module and the planned `IndexReader` trait use the word in
  this sense.
- **SearchIndex schema**: an index whose entries live in one `searchIndex`
  table (`id`, `name`, `type`, `path`). "SearchIndex" names the schema, while
  `searchIndex` in code formatting names the table itself.
- **Core Data schema**: an index stored as Apple Core Data tables (`ZTOKEN`,
  `ZTOKENTYPE`, `ZTOKENMETAINFORMATION`, `ZFILEPATH`, ...), where each entry is
  assembled through joins and the anchor is a separate column.

Earlier notes called these "schema A" (SearchIndex) and "schema B" (Core
Data).
