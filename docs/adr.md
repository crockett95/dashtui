# Architecture decisions

The design choices that shape dashtui's structure, with the alternatives that
were considered. Smaller implementation choices live in the code and its doc
comments, not here.

Each entry is numbered so code and docs can cite it (`ADR-3`). Numbers are
never reused. A decision that gets reversed stays in the list, marked
superseded and pointing to whatever replaced it.

## ADR-1: Render HTML in two stages

**Decision:** HTML is parsed once into an intermediate representation
(`Vec<Block>`, with `Inline` spans for text, code, emphasis and links).
Laying that out at a given terminal width is a separate, pure function.

**Why:** a terminal resize then only re-runs layout, without parsing the
page again. Tree-walking and line-wrapping can also be tested separately.

**Rejected:** a single HTML-to-lines pass. It's less code at first, but it
mixes tree-walking with wrapping, and every resize means parsing again.

## ADR-2: Centralized app state

**Decision:** `App::update(&mut self, msg: Msg)` is the only thing that
changes state. `ui::draw(&App)` only reads it. This is the Elm/ratatui
pattern.

**Why:** all state changes happen in one place, and that place can be unit
tested without a terminal.

**Rejected:** handling each event inline in the main loop. It's quicker to
start, but it mixes state logic with terminal I/O.

## ADR-3: Index readers behind a trait object

**Decision:** docsets use one of two on-disk index schemas: a flat
`searchIndex` table, or a Core Data schema that needs joins. Both are read
through one `IndexReader` trait, handled as `Box<dyn IndexReader>`.

**Why:** discovery returns a mix of both schemas in one list. Adding a third
schema means one new implementation, not editing every call site.

**Rejected:** a closed enum, which is fine for two variants but every `match`
has to change when a third appears. Also rejected: a bare generic, because a
mixed list still ends up boxed.

## ADR-4: Threads and channels, not async

**Decision:** index loading and search run on a worker thread and talk to the
UI over `std::sync::mpsc`.

**Why:** all I/O is local: files and SQLite. Async helps with many concurrent
network waits, and this program has none.

**Rejected:** an async runtime (`tokio`). It adds a dependency and a lot of
complexity for no benefit to this workload.

## ADR-5: `--json` output is a tested public API

**Decision:** every CLI subcommand (`list`, `stats`, `search`, ...) gets a
`--json` flag when it's written. Library functions return domain data, and
`main.rs` chooses text or JSON.

- **The JSON shape is a public API.** Changing it is a breaking change, and
  tests pin it down.
- **Domain types may serialize themselves but not reshape themselves.** A
  domain type can derive `Serialize` and carry serde attributes such as
  `#[serde(rename)]` in its definition. Any hand-written `impl Serialize`
  goes in `src/json.rs`, not next to the type. A domain type never changes
  its design to suit the JSON. If the right domain shape and the right wire
  shape differ, the JSON side adapts, through a custom impl or a view struct.
- **Every response says what it's about,** without needing the command that
  produced it. For example, `stats` output names the docset it counted
  instead of returning bare counts.

**Why:** other tools, such as an editor plugin, call `dashtui` as a
subprocess and parse the output, so a change to that output breaks them
without any warning. Output that says what it's about can be logged, cached
or compared without keeping the command line that produced it.

**Rejected:** a view struct for every domain type. That duplicates every
field while the two shapes are the same. Also rejected: text-only output with
JSON added later, which costs more than adding the flag when each command is
written.

## ADR-6: Discovery keeps docsets whose metadata won't parse

**Decision:** `find_docsets` returns one uniform `Docset { path, meta:
Option<DocsetMeta> }` for every `*.docset` directory it finds. If
`Info.plist` can't be read, the entry gets `meta: None` and is still
returned. `list` hides those entries by default, and `--include-broken`
shows them by path.

**Why:** a broken docset is still a docset on disk. The caller decides
whether to show it, and nothing is dropped silently.

**Rejected:** a `Named`/`Broken` enum, which gives two different shapes to
what is really one kind of thing. Also rejected: falling back to the raw path
with no sign that the entry is broken.

## ADR-7: `EntryType` is a small canonical enum with a catch-all

**Decision:** `EntryType` has variants only for the labels real docsets need
(`Function`, `Macro`, `Variable`, and so on), plus `Other(String)`.
Conversion is total (`From<&str>`). Known aliases such as `func` → `Function`
are normalized first, and the alias list follows Zeal's
`Docset::parseSymbolType`. Anything unrecognized is kept unchanged in
`Other`. See [entry-types.md](entry-types.md) for the observed vocabulary.

**Why:** a survey of real docsets found that a single docset can mix several
incompatible naming conventions. Mapping everything onto a large, unverified
vocabulary risks merging types that aren't actually synonyms. An enum still
supports `match`, and gives per-kind behavior somewhere to live.

**Rejected:** a closed enum covering Dash's full informal vocabulary, which
risks those wrong merges. Also rejected: a bare `String`, which loses
`match`-driven design.
