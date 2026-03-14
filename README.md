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
tryluck coin          # flip once, print heads or tails
tryluck coin 3        # flip 3 times
tryluck coin --boolean        # print true or false
tryluck coin --boolean 3      # flip 3 times in boolean mode
tryluck coin --json   # output as JSON array
```

Example output:

```
heads
tails
heads
```

### Dice roll

```sh
tryluck dice          # roll 1d6
tryluck dice 3d6      # roll 3d6, show total
tryluck dice d20      # roll 1d20
tryluck dice 2d8+2    # roll 2d8 with +2 modifier, show total
tryluck dice 3 -d 10  # roll 3d10
tryluck dice --sum    # roll 1d6, show total
tryluck dice --json   # output as JSON
```

Example output:

```
4
2
6
Total: 12
```

### Tarot draw

```sh
tryluck tarot         # draw 1 card
tryluck tarot 3       # draw 3 cards
tryluck tarot --json  # output as JSON array of {card, orientation} objects
tryluck tarot --case proper   # use proper case (e.g. "The Fool")
```

Example output:

```
The Fool
ecitsuJ
The Star
```

*(Reversed cards are displayed as reversed text)*

## MCP Server

Start the MCP server using stdio transport. Compatible with Claude Desktop and other MCP clients.

```sh
tryluck mcp
```

### Tools

| Tool | Description |
|------|-------------|
| `coin` | Flip a coin one or more times. Accepts `count` and `boolean`. Returns a JSON array of strings. |
| `dice` | Roll dice. Accepts `notation` (e.g. `"3d6+2"`), `count`, `sides`, `modifier`, `sum`. Returns a JSON array or `{rolls, total}` object. |
| `tarot` | Draw Major Arcana tarot cards. Accepts `count` and `case` (`"snake"` or `"proper"`). Returns a JSON array of `{card, orientation}` objects. |

## License

MIT OR Apache-2.0
