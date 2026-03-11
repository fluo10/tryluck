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
    /// Start the MCP server (stdio transport)
    Mcp,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Coin { boolean, count } => commands::coin::run(count.unwrap_or(1), boolean),
        Command::Mcp => commands::mcp::run().await?,
    }

    Ok(())
}
