use rand::RngExt as _;
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

// ── tool implementations ──────────────────────────────────────────────────────

#[tool_router]
impl TryluckServer {
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
}

#[tool_handler]
impl ServerHandler for TryluckServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions(
                "Tryluck provides randomization tools for TRPG and games. \
                 Use coin to flip a coin."
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
