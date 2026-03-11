# Changelog

## [0.1.0] - 2026-03-12

### Added

- `coin` command: flip a coin and print `heads` or `tails`
  - `--boolean` flag: output `true`/`false` instead of `heads`/`tails`
  - `count` argument: flip multiple times
- `mcp` command: start an MCP server (stdio transport) exposing the coin flip tool
- CI workflow: run `cargo test` on Linux (x86_64, aarch64), macOS, and Windows for every pull request
- Release workflow: build binaries for Linux (x86_64, aarch64), macOS (aarch64), and Windows (x86_64), then publish a GitHub Release when a `release/*` branch is merged to `main`
- Install scripts for Linux/macOS (`install.sh`) and Windows (`install.ps1`)
