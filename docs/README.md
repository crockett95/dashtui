# Documentation

## Design

- **[architecture.md](architecture.md)**: how the pieces fit together. It
  covers the flow from discovery through index reading and rendering to the TUI,
  plus the module layout as it stands and as planned.
- **[adr.md](adr.md)**: architecture decision records, the structural choices
  and the alternatives that were rejected. Code and docs cite these as
  `ADR-n`.
  - [ADR-1](adr.md#adr-1-render-html-in-two-stages): render HTML in two stages
  - [ADR-2](adr.md#adr-2-centralized-app-state): centralized app state
  - [ADR-3](adr.md#adr-3-index-readers-behind-a-trait-object): index readers
    behind a trait object
  - [ADR-4](adr.md#adr-4-threads-and-channels-not-async): threads and channels,
    not async
  - [ADR-5](adr.md#adr-5---json-output-is-a-tested-public-api): `--json` output
    is a tested public API
  - [ADR-6](adr.md#adr-6-discovery-keeps-docsets-whose-metadata-wont-parse):
    discovery keeps docsets whose metadata won't parse
  - [ADR-7](adr.md#adr-7-entrytype-is-a-small-canonical-enum-with-a-catch-all):
    `EntryType` is a small canonical enum with a catch-all

## Docset format reference

Dash's docset format has no formal spec. These pages record what was checked
against real docsets.

- **[docset-format.md](docset-format.md)**: the `Info.plist` keys: which ones
  every docset has, which are optional, and which dashtui models.
- **[entry-types.md](entry-types.md)**: the free-text entry `type` values
  found across 23 real docsets, Zeal's alias table, and how they map onto
  `EntryType`.
