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
    /// Start the MCP server (stdio transport)
    Mcp,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Coin { boolean, count } => commands::coin::run(count.unwrap_or(1), boolean),
        Command::Tarot { json, case, count } => {
            use commands::tarot::Case;
            let case = case.unwrap_or(if json { Case::Snake } else { Case::Proper });
            commands::tarot::run(count.unwrap_or(1), json, case)
        }
        Command::Mcp => commands::mcp::run().await?,
    }

    Ok(())
}
