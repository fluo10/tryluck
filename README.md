# tryluck

Try your luck with AI: randomized tarot, dice, and coin results for creative storytelling and TRPG adventures via CLI and MCP.

## Installation

**Linux / macOS:**

```sh
curl -fsSL https://raw.githubusercontent.com/fluo10/tryluck/main/install.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://raw.githubusercontent.com/fluo10/tryluck/main/install.ps1 | iex
```

**From source:**

```sh
cargo install --path .
```

## CLI

### Coin flip

```sh
tryluck coin          # print heads or tails once
tryluck coin 3        # flip 3 times
tryluck coin --boolean        # print true or false
tryluck coin --boolean 3      # flip 3 times in boolean mode
```

Example output:

```
heads
tails
heads
```

## MCP Server

Start the MCP server using stdio transport. Compatible with Claude Desktop and other MCP clients.

```sh
tryluck mcp
```

### Tools

| Tool | Description |
|------|-------------|
| `coin` | Flip a coin one or more times. Accepts `count` (number of flips) and `boolean` (return `true`/`false` instead of `heads`/`tails`). Returns a JSON array of strings. |

## Planned Features

> **Note:** Only `coin` is available in the current release. The following features are under development.

- `tryluck dice` — dice rolls with notation like `1d6`, `2d10`
- `tryluck tarot` — random tarot card draws
- MCP tools for the above

## License

MIT OR Apache-2.0