# Docset `Info.plist` key reference

There's no formal, versioned spec for the Dash docset format — Apple defines the
plist format itself, but the specific keys below are Dash's own convention,
documented loosely at [kapeli.com/docsets](https://kapeli.com/docsets) and
otherwise only discoverable by reading real docsets. This page combines that
guide with empirical checks against the real docsets in `docsets/` (gitignored,
downloaded in M0) and the crafted fixtures in `tests/fixtures/meta/`.

**Verified 2026-09-27** against `Bash.docset`, `SQLite.docset`, `Lua.docset`
(the same three docsets profiled for index schema in `PROGRESS.md`, M0).

## Baseline keys (present in every real docset checked)

| Key | Type | Meaning |
| --- | --- | --- |
| `CFBundleIdentifier` | string | The docset's own unique identity — the standard Apple bundle identifier, reverse-DNS style in principle (`org.nsnam.ns3`) though often just a lowercase short name in practice (`bash`, `lua`, `sqlite` in our corpus). Independent of the display name and independent of the search keyword — see `DocSetPlatformFamily` below. Not guaranteed globally unique across all docsets in the wild (Dash doesn't enforce a registry), but meant to be stable for one docset across versions. |
| `CFBundleName` | string | Human-readable display name shown in Dash's docset list (`Bash`, `SQLite`, `Lua 5.5`). |
| `DocSetPlatformFamily` | string | The search-scoping keyword — what you type before `:` in Dash's search bar to restrict results to this docset (or this docset's family). Usually matches `CFBundleIdentifier`, but the two are allowed to diverge: Flutter's real docset uses `CFBundleIdentifier: flutter` but `DocSetPlatformFamily: dartlang`, so it's grouped and searchable under the broader Dart family rather than standing alone. |

`DocsetMeta` (decision 6 in `PLAN.md`) models all three as required `String`
fields — every docset checked had them, with no observed exceptions.

## `dashIndexFilePath` (present in some, not all)

| Key | Type | Meaning |
| --- | --- | --- |
| `dashIndexFilePath` | string (path) | Path to the page Dash opens first when you open the docset, relative to `Contents/Resources/Documents/`. |

Observed as present in `Bash.docset` (`bash/index.html`) and `Lua.docset`
(`www.lua.org/manual/5.5/contents.html`), but **absent** from `SQLite.docset`
— the real-world case that drove decision 6 to model this as
`Option<PathBuf>` rather than a required field.

## Other keys seen or documented, not modeled by `DocsetMeta` (yet)

Nothing in the project currently reads these — listed for awareness, per
decision 6's note that `DocsetMeta` only carries fields something actually
uses.

| Key | Type | Meaning |
| --- | --- | --- |
| `isDashDocset` | bool (`<true/>`) | Marks the bundle as a valid Dash docset. In the Kapeli sample template, and present in `Bash.docset` and `SQLite.docset` — but **absent from `Lua.docset`**, so it isn't reliably present either. |
| `DashDocSetFamily` | string | Opts into extra navigation behavior. Kapeli's guide documents the value `dashtoc` for anchor-based table-of-contents support (seen in `Lua.docset`). `Bash.docset` uses `unsorteddashtoc` instead — a real variant not mentioned on the official page, presumably a ToC without alphabetical sorting. Treat this key's value set as open-ended, not just the one documented value. |
| `DashDocSetFallbackURL` | string (URL) | Base URL for redirecting to online docs when local content is missing. Not observed in the local corpus. |
| `DashDocSetPlayURL` | string (URL) | Link to an online interactive playground for the docset's language. Not observed locally. |
| `isJavaScriptEnabled` | bool | Opts into running external `.js` from rendered pages; disabled by default. Not observed locally — also moot for this project, since JavaScript execution is an explicit non-goal (`PLAN.md`). |
| `DashDocSetDefaultFTSEnabled` | bool | Turns on full-text search by default for this docset. Not observed locally. |
| `DashDocSetFTSNotSupported` | bool | Disables full-text search entirely for this docset. Not observed locally. |

## Caveats

- This isn't exhaustive. Dash has no published schema, so an unfamiliar key in
  some other docset is entirely possible — treat silence here as "not yet
  seen," not "doesn't exist."
- Values are strings/bools as XML plist types; a Core-Data-schema docset (see
  `CLAUDE.md`'s docset format notes) uses a different mechanism for its
  *index*, but still uses this same `Info.plist` format for metadata.

## Sources

- [Docset Generation Guide — kapeli.com/docsets](https://kapeli.com/docsets)
  and its linked [sample `Info.plist`](https://kapeli.com/resources/Info.plist)
- [Flutter issue #97386](https://github.com/flutter/flutter/issues/97386) —
  real-world example of `CFBundleIdentifier`/`DocSetPlatformFamily` diverging
- Real docsets in `docsets/` (gitignored): `Bash.docset`, `SQLite.docset`,
  `Lua.docset`
- Fixtures in `tests/fixtures/meta/`, built for the `DocsetMeta` parser's tests
