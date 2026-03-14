use rand::RngExt as _;
use crate::commands::tarot::{self, Case};
use crate::commands::dice;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
    transport::stdio,
};
use serde::Deserialize;

// ── server struct ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
struct TryluckServer {
    tool_router: ToolRouter<Self>,
}

impl TryluckServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

// ── parameter structs ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct CoinParams {
    /// Number of flips (default: 1)
    count: Option<u32>,
    /// Output true/false instead of heads/tails (default: false)
    boolean: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct DiceParams {
    /// Dice notation (e.g. "3d10", "d6", "3d10+2"). Overridden by explicit count/sides/modifier.
    notation: Option<String>,
    /// Number of sides (default: 6)
    sides: Option<u32>,
    /// Number of rolls (default: 1)
    count: Option<u32>,
    /// Modifier added to total (default: 0)
    modifier: Option<i32>,
    /// Return {rolls, total} instead of a plain array (implied when modifier is non-zero)
    sum: Option<bool>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct TarotParams {
    /// Number of cards to draw (default: 1, max: 22)
    count: Option<u32>,
    /// Card name case: "snake" (default, e.g. "the_fool") or "proper" (e.g. "The Fool")
    case: Option<String>,
}

// ── tool implementations ──────────────────────────────────────────────────────

#[tool_router]
impl TryluckServer {
    #[tool(description = "Roll dice one or more times. Returns a JSON array of roll results normally, or an object {\"rolls\": [...], \"total\": N} when sum=true or modifier is non-zero. Supports dice notation (e.g. \"3d10\", \"d6+2\") or explicit count/sides/modifier parameters.")]
    fn dice(&self, Parameters(p): Parameters<DiceParams>) -> Result<String, String> {
        let (mut count, mut sides, mut modifier) = (1u32, 6u32, 0i32);
        let notation_used = p.notation.as_deref().map_or(false, |n| n.to_ascii_lowercase().contains('d'));
        if let Some(ref notation) = p.notation {
            let (c, s, m) = dice::parse_spec(notation);
            if let Some(c) = c { count = c; }
            if let Some(s) = s { sides = s; }
            if let Some(m) = m { modifier = m; }
        }
        if let Some(c) = p.count { count = c; }
        if let Some(s) = p.sides { sides = s; }
        if let Some(m) = p.modifier { modifier = m; }

        let show_total = p.sum.unwrap_or(false) || modifier != 0 || notation_used;
        let rolls: Vec<u32> = (0..count).map(|_| rand::rng().random_range(1..=sides)).collect();
        let total: i64 = rolls.iter().map(|&r| r as i64).sum::<i64>() + modifier as i64;

        if show_total {
            rmcp::serde_json::to_string(&rmcp::serde_json::json!({ "rolls": rolls, "total": total }))
                .map_err(|e| e.to_string())
        } else {
            rmcp::serde_json::to_string(&rolls).map_err(|e| e.to_string())
        }
    }

    #[tool(description = "Flip a coin one or more times. Returns a JSON array of heads/tails strings, or true/false strings when boolean=true.")]
    fn coin(&self, Parameters(p): Parameters<CoinParams>) -> Result<String, String> {
        let count = p.count.unwrap_or(1).max(1);
        let boolean = p.boolean.unwrap_or(false);
        let results: Vec<&str> = (0..count)
            .map(|_| {
                let result = rand::rng().random_bool(0.5);
                if boolean {
                    if result { "true" } else { "false" }
                } else {
                    if result { "heads" } else { "tails" }
                }
            })
            .collect();
        rmcp::serde_json::to_string(&results).map_err(|e: rmcp::serde_json::Error| e.to_string())
    }

    #[tool(description = "Draw one or more Major Arcana tarot cards. Returns a JSON array of objects with `card` (card name) and `orientation` (\"upright\" or \"reversed\") fields. Card names use snake_case by default (e.g. \"the_fool\"), or proper case when case=\"proper\".")]
    fn tarot(&self, Parameters(p): Parameters<TarotParams>) -> Result<String, String> {
        let count = p.count.unwrap_or(1);
        let case = match p.case.as_deref() {
            Some("proper") => Case::Proper,
            _ => Case::Snake,
        };
        let draws = tarot::draw(count, case);
        rmcp::serde_json::to_string(&draws).map_err(|e: rmcp::serde_json::Error| e.to_string())
    }
}

#[tool_handler]
impl ServerHandler for TryluckServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "Tryluck provides randomization tools for TRPG and games. \
                 Use coin to flip a coin. \
                 Use dice to roll dice (supports notation like 3d10+2). \
                 Use tarot to draw Major Arcana tarot cards."
                    .to_owned(),
            )
    }
}

// ── main ──────────────────────────────────────────────────────────────────────

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_writer(std::io::stderr).init();

    let server = TryluckServer::new();
    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
