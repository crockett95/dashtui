# dashtui

A keyboard-driven terminal browser for [Dash](https://kapeli.com/dash) /
[Zeal](https://zealdocs.org) docsets. You'll be able to search a docset's index,
read the page as wrapped and styled text in the terminal, follow links, and go
back.

> **Status: early.** Only docset discovery (`list`) works so far. Index reading,
> search, and the TUI are in progress. See [docs/architecture.md](docs/architecture.md)
> for the planned design.
> **Everything subject to change before v1**

**NB**: _This is a project for me to learn
[`Rust`](https://doc.rust-lang.org/book/) with an example that I like, so it's
being done at my own leisurely pace. Feedback is welcome, but let me have my
fun with it._

## Getting docsets

dashtui reads docsets that are already on disk; it doesn't download them. Any
`*.docset` directory works, including ones Dash or Zeal has already downloaded.
To fetch one by hand, use the mirror URLs listed in each docset's feed in
[Kapeli/feeds](https://github.com/Kapeli/feeds):

```sh
mkdir -p docsets
curl -L https://london.kapeli.com/feeds/Bash.tgz | tar -xz -C docsets
```

`docsets/` is gitignored, so you can keep docsets inside the repo while you
develop.

## Usage

```sh
cargo run -- list docsets
```

```
Bash    docsets/Bash.docset
Lua 5.5 docsets/Lua.docset
```

`--include-broken` also lists docsets whose `Info.plist` can't be read, shown
by path. `--json` prints JSON instead of text:

```sh
cargo run -- --json list docsets
```

```json
{"docsets":[{"path":"docsets/Bash.docset","id":"bash","name":"Bash","platform_family":"bash","index_file":"bash/index.html"}]}
```

The JSON output is a stable, tested interface for scripts and editor plugins
([ADR-5](docs/adr.md#adr-5---json-output-is-a-tested-public-api)).

## Development

```sh
cargo test                   # needs no docsets; the tests use tests/fixtures/
cargo fmt --check
cargo clippy --all-targets
```

The repo includes a devcontainer config (`.devcontainer/`).

The `.claude` submodule points at a private repo of the author's working notes.
You don't need it: a plain `git clone` leaves the directory empty, and
everything builds and tests without it.

## Documentation

See [docs/](docs/README.md) for the architecture, design decisions, and
reference notes on the docset format.

## License

[Mozilla Public License 2.0](LICENSE.md).
