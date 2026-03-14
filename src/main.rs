mod commands;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tryluck", about = "Divination randomizer: tarot, dice, coins via CLI and MCP", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Flip a coin and print heads or tails
    Coin {
        /// Output as JSON array of {result} objects
        #[arg(long)]
        json: bool,
        /// Output true/false instead of heads/tails
        #[arg(long)]
        boolean: bool,
        /// Number of flips (default: 1)
        count: Option<u32>,
    },
    /// Draw tarot cards from the Major Arcana
    Tarot {
        /// Output as JSON array of {card, orientation} objects (defaults to snake case)
        #[arg(long)]
        json: bool,
        /// Card name case format (default: proper for plain text, snake for --json)
        #[arg(long, value_enum)]
        case: Option<commands::tarot::Case>,
        /// Number of cards to draw (default: 1)
        count: Option<u32>,
    },
    /// Roll dice
    Dice {
        /// Dice notation (e.g. 3d10, d6, 3d10+2, 2d8-1) or plain count (e.g. 3)
        spec: Option<String>,
        /// Number of sides (default: 6, overrides notation)
        #[arg(short = 'd', long)]
        sides: Option<u32>,
        /// Modifier added to total (default: 0, overrides notation)
        #[arg(short = 'm', long)]
        modifier: Option<i32>,
        /// Output sum of all rolls (implied when modifier is non-zero)
        #[arg(long)]
        sum: bool,
        /// Output as JSON array, or {rolls, total} object when summing
        #[arg(long)]
        json: bool,
    },
    /// Start the MCP server (stdio transport)
    Mcp,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Coin { json, boolean, count } => commands::coin::run(count.unwrap_or(1), boolean, json),
        Command::Tarot { json, case, count } => {
            use commands::tarot::Case;
            let case = case.unwrap_or(if json { Case::Snake } else { Case::Proper });
            commands::tarot::run(count.unwrap_or(1), json, case)
        }
        Command::Dice { spec, sides, modifier, sum, json } => {
            let (mut count, mut parsed_sides, mut parsed_modifier) = (1u32, 6u32, 0i32);
            let mut notation_used = false;
            if let Some(spec) = spec {
                notation_used = spec.to_ascii_lowercase().contains('d');
                let (c, s, m) = commands::dice::parse_spec(&spec);
                if let Some(c) = c { count = c; }
                if let Some(s) = s { parsed_sides = s; }
                if let Some(m) = m { parsed_modifier = m; }
            }
            commands::dice::run(count, sides.unwrap_or(parsed_sides), modifier.unwrap_or(parsed_modifier), sum || notation_used, json)
        }
        Command::Mcp => commands::mcp::run().await?,
    }

    Ok(())
}
