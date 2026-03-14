# Changelog

## [0.2.0] - 2026-03-15

### Added

- `dice` command: roll dice with TRPG-style notation (e.g. `3d6`, `d20`, `2d8+2`)
  - `spec` argument: dice notation or plain count
  - `-d/--sides` flag: number of sides (default: 6)
  - `-m/--modifier` flag: modifier added to total
  - `--sum` flag: output sum of all rolls
  - `--json` flag: output as JSON array or `{rolls, total}` object when summing
  - Total is shown automatically when using dice notation or a modifier
- `tarot` command: draw Major Arcana tarot cards
  - `count` argument: number of cards to draw (default: 1, max: 22)
  - `--json` flag: output as JSON array of `{card, orientation}` objects
  - `--case` flag: card name format (`proper` e.g. "The Fool", `snake` e.g. "the_fool")
  - Reversed cards shown as reversed text in plain output
- `coin --json` flag: output coin flip results as a JSON array
- MCP tools: `dice` and `tarot` tools added to the MCP server

## [0.1.0] - 2026-03-12

### Added

- `coin` command: flip a coin and print `heads` or `tails`
  - `--boolean` flag: output `true`/`false` instead of `heads`/`tails`
  - `count` argument: flip multiple times
- `mcp` command: start an MCP server (stdio transport) exposing the coin flip tool
- CI workflow: run `cargo test` on Linux (x86_64, aarch64), macOS, and Windows for every pull request
- Release workflow: build binaries for Linux (x86_64, aarch64), macOS (aarch64), and Windows (x86_64), then publish a GitHub Release when a `release/*` branch is merged to `main`
- Install scripts for Linux/macOS (`install.sh`) and Windows (`install.ps1`)
